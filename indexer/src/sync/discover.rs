use super::builder::build_plugins_from_nukkit;
use crate::github::client;
use crate::plugin::Plugin;
use chrono::{Datelike, Utc};
use std::collections::{HashMap, HashSet};
use tracing::{debug, debug_span, info, info_span, warn};

const CODE_SEARCH_QUERY: &str = "filename:plugin.yml path:src/main/resources language:YAML";

const TOPIC_QUERIES: &[&str] = &[
    "topic:nukkit-plugin fork:true",
    "topic:nukkit-mot-plugin fork:true",
];

const EXCLUDED_REPOS: &[&str] = &[];

const START_YEAR: i32 = 2015;
const SHARD_LIMIT: u64 = 1000;

#[derive(Debug, Clone)]
struct RepoMatch {
    full_name: String,
    plugin_yml_path: Option<String>,
}

pub struct DiscoverResult {
    pub new_plugins: Vec<Plugin>,
    pub errors: Vec<(String, String)>,
}

pub fn discover_new_plugins(
    existing_ids: &HashSet<String>,
    existing_repos: &HashSet<String>,
    last_sync: Option<&str>,
) -> DiscoverResult {
    let mut matches = {
        let _span = info_span!("collect_repos").entered();
        match last_sync {
            Some(date) => {
                let query = format!("{} pushed:>{}", CODE_SEARCH_QUERY, date);
                let mut matches = collect_repo_matches(&query, existing_repos).unwrap_or_default();
                let topic_matches = collect_repo_matches_by_topic(existing_repos, Some(date));
                merge_repo_matches(&mut matches, topic_matches);
                matches
            }
            None => collect_repo_matches_full(existing_repos),
        }
    };

    // Append repos from motci.cn Jenkins
    let jenkins_matches = collect_repo_matches_from_jenkins(existing_repos, &matches);
    if !jenkins_matches.is_empty() {
        info!(count = jenkins_matches.len(), "Found repos via motci.cn");
        merge_repo_matches(&mut matches, jenkins_matches);
    }

    info!(count = matches.len(), "Found repos to process");
    if matches.is_empty() {
        return DiscoverResult {
            new_plugins: Vec::new(),
            errors: Vec::new(),
        };
    }

    let _span = info_span!("process_repos", count = matches.len()).entered();
    process_repos_parallel(matches, existing_ids)
}

fn collect_repo_matches_full(existing_repos: &HashSet<String>) -> Vec<RepoMatch> {
    let mut matches = match collect_repo_matches(CODE_SEARCH_QUERY, existing_repos) {
        Ok(m) => m,
        Err(total) => {
            info!(
                total = total,
                "Results exceed 1000, using year-based sharding"
            );
            collect_repo_matches_by_year(existing_repos)
        }
    };

    let topic_matches = collect_repo_matches_by_topic(existing_repos, None);
    info!(
        code_count = matches.len(),
        topic_count = topic_matches.len(),
        "Merging code search and topic search results"
    );
    merge_repo_matches(&mut matches, topic_matches);

    matches
}

fn collect_repo_matches_by_year(existing_repos: &HashSet<String>) -> Vec<RepoMatch> {
    let current_year = Utc::now().year();
    let mut repo_map: HashMap<String, Vec<String>> = HashMap::new();

    for year in START_YEAR..=current_year {
        let _span = debug_span!("search_year", year = year).entered();
        let query = format!(
            "{} pushed:{}-01-01..{}-12-31",
            CODE_SEARCH_QUERY, year, year
        );
        let matches = match collect_repo_matches(&query, existing_repos) {
            Ok(m) => m,
            Err(total) => {
                warn!(year = year, total = total, "Year truncated (> 1000)");
                continue;
            }
        };

        for m in matches {
            if let Some(path) = m.plugin_yml_path {
                repo_map.entry(m.full_name).or_default().push(path);
            }
        }
    }

    repo_map
        .into_iter()
        .map(|(full_name, paths)| RepoMatch {
            full_name,
            plugin_yml_path: paths.into_iter().next(),
        })
        .collect()
}

fn collect_repo_matches_by_topic(
    existing_repos: &HashSet<String>,
    since: Option<&str>,
) -> Vec<RepoMatch> {
    let mut repo_map: HashMap<String, Vec<String>> = HashMap::new();

    for topic_query in TOPIC_QUERIES {
        let query = if let Some(date) = since {
            format!("{} pushed:>{}", topic_query, date)
        } else {
            topic_query.to_string()
        };

        for page in 1..=10 {
            let _span = debug_span!("topic_search", query = %query, page = page).entered();
            match client().search_repositories(&query, page) {
                Ok(result) => {
                    if result.items.is_empty() {
                        break;
                    }

                    for item in &result.items {
                        let name = &item.full_name;
                        if item.fork && !topic_query.contains("fork:true") {
                            debug!(repo = %name, "Skip fork");
                            continue;
                        }
                        if existing_repos.contains(name) {
                            debug!(repo = %name, "Skip existing");
                            continue;
                        }
                        if EXCLUDED_REPOS.contains(&name.as_str()) {
                            debug!(repo = %name, "Skip excluded");
                            continue;
                        }
                        repo_map.entry(name.clone()).or_default();
                    }

                    if result.items.len() < 100 {
                        break;
                    }
                }
                Err(e) => {
                    warn!(error = %e, page = page, query = %query, "Topic search error");
                    break;
                }
            }
        }
    }

    info!(count = repo_map.len(), "Found repos via topic search");

    repo_map
        .into_iter()
        .map(|(full_name, _)| RepoMatch {
            full_name,
            plugin_yml_path: None,
        })
        .collect()
}

fn merge_repo_matches(base: &mut Vec<RepoMatch>, additions: Vec<RepoMatch>) {
    let existing: HashSet<_> = base.iter().map(|m| m.full_name.clone()).collect();

    for addition in additions {
        if !existing.contains(&addition.full_name) {
            base.push(addition);
        }
    }
}

fn collect_repo_matches(
    query: &str,
    existing_repos: &HashSet<String>,
) -> Result<Vec<RepoMatch>, u64> {
    let first = match client().search_code(query, 1) {
        Ok(r) => r,
        Err(e) => {
            warn!(error = %e, "Search error");
            return Ok(Vec::new());
        }
    };

    if first.total_count > SHARD_LIMIT {
        return Err(first.total_count);
    }

    let mut repo_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut process_items = |items: &[crate::github::CodeSearchItem]| {
        for item in items {
            let name = &item.repository.full_name;
            if item.repository.fork {
                debug!(repo = %name, "Skip fork");
                continue;
            }
            if existing_repos.contains(name) {
                debug!(repo = %name, "Skip existing");
                continue;
            }
            if EXCLUDED_REPOS.contains(&name.as_str()) {
                debug!(repo = %name, "Skip excluded");
                continue;
            }
            repo_map
                .entry(name.clone())
                .or_default()
                .push(item.path.clone());
        }
    };

    process_items(&first.items);

    if first.items.len() >= 100 {
        for page in 2..=10 {
            match client().search_code(query, page) {
                Ok(result) => {
                    if result.items.is_empty() {
                        break;
                    }
                    process_items(&result.items);
                    if result.items.len() < 100 {
                        break;
                    }
                }
                Err(e) => {
                    warn!(error = %e, page = page, "Search error");
                    break;
                }
            }
        }
    }

    Ok(repo_map
        .into_iter()
        .map(|(full_name, paths)| RepoMatch {
            full_name,
            plugin_yml_path: paths.into_iter().find(|p| p.ends_with("plugin.yml")),
        })
        .collect())
}

fn process_repos_parallel(
    matches: Vec<RepoMatch>,
    existing_ids: &HashSet<String>,
) -> DiscoverResult {
    let batch = client().execute_parallel(matches, |repo_match, _| {
        let _span = debug_span!("process_repo", repo = %repo_match.full_name).entered();
        let full_name = repo_match.full_name.clone();
        (full_name, process_single_repo(repo_match))
    });

    if batch.stopped_by_rate_limit {
        warn!(
            processed = batch.processed,
            total = batch.total,
            "Stopped early due to rate limit"
        );
    }

    let mut seen_ids: HashSet<String> = existing_ids.clone();
    let mut new_plugins = Vec::new();
    let mut errors = Vec::new();

    for (full_name, res) in batch.results {
        match res {
            Ok(plugins) => {
                for plugin in plugins {
                    if !seen_ids.contains(&plugin.id) {
                        seen_ids.insert(plugin.id.clone());
                        new_plugins.push(plugin);
                    } else {
                        debug!(id = %plugin.id, repo = %full_name, "Skip duplicate ID");
                    }
                }
            }
            Err(e) => {
                errors.push((full_name, e));
            }
        }
    }

    DiscoverResult {
        new_plugins,
        errors,
    }
}

fn process_single_repo(repo_match: RepoMatch) -> Result<Vec<Plugin>, String> {
    let parts: Vec<&str> = repo_match.full_name.split('/').collect();
    if parts.len() != 2 {
        return Err("invalid repo name".to_string());
    }

    let repo = client().get_repository(parts[0], parts[1])?;

    if repo.is_template {
        debug!(repo = %repo_match.full_name, "Skip template");
        return Ok(Vec::new());
    }
    if repo.archived {
        debug!(repo = %repo_match.full_name, "Skip archived");
        return Ok(Vec::new());
    }
    if repo.topics.iter().any(|t| t == "noindex") {
        debug!(repo = %repo_match.full_name, "Skip noindex");
        return Ok(Vec::new());
    }

    let plugin_yml_path = match repo_match.plugin_yml_path {
        Some(path) => path,
        None => {
            // Try to find plugin.yml in the repository
            match find_plugin_yml(parts[0], parts[1], &repo) {
                Some(path) => path,
                None => {
                    debug!(repo = %repo_match.full_name, "No plugin.yml found");
                    return Ok(Vec::new());
                }
            }
        }
    };

    let plugins = build_plugins_from_nukkit(&repo, &plugin_yml_path);
    if plugins.is_empty() {
        debug!(repo = %repo_match.full_name, "No plugins built");
    }

    Ok(plugins)
}

fn find_plugin_yml(
    owner: &str,
    repo_name: &str,
    repo: &crate::github::Repository,
) -> Option<String> {
    let branch = repo.default_branch.as_deref().unwrap_or("main");

    match client().get_tree(owner, repo_name, branch) {
        Ok(tree) => tree
            .tree
            .iter()
            .find(|entry| {
                entry.entry_type == "blob"
                    && entry.path.ends_with("plugin.yml")
                    && entry.path.contains("src/main/resources")
            })
            .map(|entry| entry.path.clone()),
        Err(e) => {
            debug!(repo = %format!("{}/{}", owner, repo_name), error = %e, "Failed to get tree");
            None
        }
    }
}

fn collect_repo_matches_from_jenkins(
    existing_repos: &HashSet<String>,
    current_matches: &[RepoMatch],
) -> Vec<RepoMatch> {
    let current: HashSet<String> = current_matches
        .iter()
        .map(|m| m.full_name.to_lowercase())
        .collect();

    crate::jenkins::jenkins_index()
        .repo_names()
        .filter(|name| {
            !existing_repos
                .iter()
                .any(|r| r.eq_ignore_ascii_case(name))
                && !current.contains(&name.to_lowercase())
        })
        .map(|name| RepoMatch {
            full_name: name.to_string(),
            plugin_yml_path: None,
        })
        .collect()
}
