use super::builder::build_plugins_from_repo;
use crate::github::client;
use crate::plugin::Plugin;
use chrono::{Datelike, Utc};
use std::collections::{HashMap, HashSet};
use tracing::{debug, debug_span, info, info_span, warn};

const CODE_SEARCH_QUERY: &str = "org.allaymc filename:build.gradle -repo:AllayMC/Allay -repo:AllayMC/StateUpdater -repo:AllayMC/EncryptMyPack -repo:AllayMC/AllayGradle -repo:AllayMC/NBT -repo:AllayMC/JavaPluginTemplate -repo:AllayPlus/AllayPlus -repo:MineBuilders/allaymc-kotlin-plugin-template -user:Buddelbubi";

const TOPIC_QUERIES: &[&str] = &["topic:allaymc-plugin fork:true"];

const EXCLUDED_REPOS: &[&str] = &[
    "AllayMC/Allay",
    "AllayMC/StateUpdater",
    "AllayMC/EncryptMyPack",
    "AllayMC/AllayGradle",
    "AllayMC/NBT",
    "AllayMC/JavaPluginTemplate",
    "AllayPlus/AllayPlus",
    "MineBuilders/allaymc-kotlin-plugin-template",
];

#[derive(Debug, Clone)]
pub struct RepoMatch {
    pub full_name: String,
    pub gradle_paths: Vec<String>,
}

const START_YEAR: i32 = 2023;
const SHARD_LIMIT: u64 = 1000;

pub struct DiscoverResult {
    pub new_plugins: Vec<Plugin>,
    pub errors: Vec<(String, String)>,
}

pub fn discover_new_plugins(
    existing_ids: &HashSet<String>,
    existing_repos: &HashSet<String>,
    last_sync: Option<&str>,
) -> DiscoverResult {
    let matches = {
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
        let query = format!("{} pushed:{}-01-01..{}-12-31", CODE_SEARCH_QUERY, year, year);
        let matches = match collect_repo_matches(&query, existing_repos) {
            Ok(m) => m,
            Err(total) => {
                warn!(year = year, total = total, "Year truncated (> 1000)");
                continue;
            }
        };

        for m in matches {
            repo_map
                .entry(m.full_name)
                .or_default()
                .extend(m.gradle_paths);
        }
    }

    repo_map
        .into_iter()
        .map(|(full_name, mut paths)| {
            paths.sort();
            paths.dedup();
            RepoMatch {
                full_name,
                gradle_paths: paths,
            }
        })
        .collect()
}

fn collect_repo_matches_by_topic(
    existing_repos: &HashSet<String>,
    pushed_after: Option<&str>,
) -> Vec<RepoMatch> {
    let mut repo_map: HashMap<String, Vec<String>> = HashMap::new();

    for topic_query in TOPIC_QUERIES {
        let _span = debug_span!("search_topic", query = %topic_query).entered();

        let query = match pushed_after {
            Some(date) => format!("{} pushed:>{}", topic_query, date),
            None => topic_query.to_string(),
        };

        for page in 1..=10 {
            match client().search_repositories(&query, page) {
                Ok(result) => {
                    for repo in &result.items {
                        let name = &repo.full_name;

                        if EXCLUDED_REPOS.iter().any(|e| e == name) {
                            debug!(repo = %name, "Skip excluded");
                            continue;
                        }
                        if existing_repos.contains(name) {
                            debug!(repo = %name, "Skip existing");
                            continue;
                        }

                        repo_map.entry(name.clone()).or_default();
                    }

                    if result.items.len() < 100 {
                        break;
                    }
                }
                Err(e) => {
                    warn!(error = %e, page = page, query = %topic_query, "Topic search error");
                    break;
                }
            }
        }
    }

    info!(count = repo_map.len(), "Found repos via topic search");

    repo_map
        .into_iter()
        .map(|(full_name, gradle_paths)| RepoMatch {
            full_name,
            gradle_paths,
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
        .map(|(full_name, gradle_paths)| RepoMatch {
            full_name,
            gradle_paths,
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

    let gradle_paths = if repo_match.gradle_paths.is_empty() {
        find_gradle_files(parts[0], parts[1], &repo)
    } else {
        repo_match.gradle_paths
    };

    if gradle_paths.is_empty() {
        debug!(repo = %repo_match.full_name, "No gradle files found");
        return Ok(Vec::new());
    }

    let plugins = build_plugins_from_repo(&repo, &gradle_paths);
    if plugins.is_empty() {
        debug!(repo = %repo_match.full_name, "No plugins found");
    }
    Ok(plugins)
}

fn find_gradle_files(owner: &str, repo_name: &str, repo: &crate::github::Repository) -> Vec<String> {
    let branch = repo.default_branch.as_deref().unwrap_or("main");

    match client().get_tree(owner, repo_name, branch) {
        Ok(tree) => {
            tree.tree
                .iter()
                .filter(|entry| {
                    entry.entry_type == "blob"
                        && (entry.path.ends_with("build.gradle.kts")
                            || entry.path.ends_with("build.gradle"))
                })
                .map(|entry| entry.path.clone())
                .collect()
        }
        Err(e) => {
            debug!(repo = %format!("{}/{}", owner, repo_name), error = %e, "Failed to get tree");
            Vec::new()
        }
    }
}
