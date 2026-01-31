mod image;
mod link;
mod version_resolver;

use crate::github::{Contributor, GitTreeEntry, Release, Repository, client};

const CATEGORIES: &[&str] = &[
    "adventure",
    "cursed",
    "decoration",
    "economy",
    "equipment",
    "food",
    "game-mechanics",
    "library",
    "magic",
    "management",
    "minigame",
    "mobs",
    "optimization",
    "social",
    "storage",
    "technology",
    "transportation",
    "utility",
    "world-generation",
];
use crate::gradle::{AllayDsl, VersionRef, parse_build_gradle_kts, parse_plugin_json};
use crate::plugin::{
    Author, Dependency, GalleryItem, License, Links, Plugin, Version, VersionFile,
};
use tracing::debug;

pub struct PostProcessContext<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub branch: &'a str,
}

struct PluginBuildInput<'a> {
    repo: &'a Repository,
    dsl: &'a AllayDsl,
    releases: &'a [Release],
    readme: &'a str,
    license: &'a License,
    contributors: &'a [Contributor],
    owner: &'a str,
    repo_name: &'a str,
    branch: &'a str,
    icon_url: &'a str,
    repo_gallery: Vec<GalleryItem>,
}

type ImageProcessorFn = fn(&str, &PostProcessContext, &mut Vec<GalleryItem>) -> String;
type LinkProcessorFn = fn(&str, &PostProcessContext) -> String;

static IMAGE_PROCESSORS: &[ImageProcessorFn] =
    &[image::process_html_images, image::process_md_images];
static LINK_PROCESSORS: &[LinkProcessorFn] = &[link::process_md_links, link::process_html_links];

fn process_readme(readme: &str, ctx: &PostProcessContext) -> (String, Vec<GalleryItem>) {
    let mut content = readme.to_string();
    let mut gallery = Vec::new();

    for processor in IMAGE_PROCESSORS {
        content = processor(&content, ctx, &mut gallery);
    }

    for processor in LINK_PROCESSORS {
        content = processor(&content, ctx);
    }

    (content, gallery)
}

fn gradle_path_to_module(path: &str) -> Option<String> {
    if let Some(dir) = path.strip_suffix("/build.gradle.kts") {
        Some(dir.to_string())
    } else if let Some(dir) = path.strip_suffix("/build.gradle") {
        Some(dir.to_string())
    } else if path == "build.gradle.kts" || path == "build.gradle" {
        Some("root".to_string())
    } else {
        None
    }
}

fn plugin_json_paths_for_module(module: &str) -> Vec<String> {
    let base = if module == "root" || module.is_empty() {
        "src/main/resources".to_string()
    } else {
        format!("{}/src/main/resources", module)
    };
    vec![
        format!("{}/plugin.json", base),
        format!("{}/extension.json", base),
    ]
}

fn get_tree(owner: &str, repo: &str, branch: &str) -> Vec<GitTreeEntry> {
    client()
        .get_tree(owner, repo, branch)
        .map(|t| t.tree)
        .unwrap_or_default()
}

fn find_gradle_paths_from_tree(tree: &[GitTreeEntry]) -> Vec<String> {
    tree.iter()
        .filter(|e| {
            e.entry_type == "blob"
                && (e.path.ends_with("build.gradle.kts"))
        })
        .map(|e| e.path.clone())
        .collect()
}

fn is_branch_snapshot(version: &str) -> bool {
    let lower = version.to_lowercase();
    lower.ends_with("-snapshot") && !lower.chars().next().is_some_and(|c| c.is_ascii_digit())
}

fn resolve_dsl_versions(dsl: &mut AllayDsl, tree: &[GitTreeEntry], owner: &str, repo: &str) {
    if !matches!(dsl.api_version_ref, VersionRef::Literal(_))
        && let Some(v) = version_resolver::resolve_version(&dsl.api_version_ref, tree, owner, repo)
        {
            dsl.api = Some(v);
        }

    if !matches!(dsl.server_version_ref, VersionRef::Literal(_))
        && let Some(v) =
            version_resolver::resolve_version(&dsl.server_version_ref, tree, owner, repo)
        {
            dsl.server = Some(v);
        }

    if let Some(api) = &dsl.api
        && is_branch_snapshot(api)
            && let Some(v) = version_resolver::resolve_snapshot_version() {
                dsl.api = Some(v);
            }

    if let Some(server) = &dsl.server
        && is_branch_snapshot(server)
            && let Some(v) = version_resolver::resolve_snapshot_version() {
                dsl.server = Some(v);
            }
}

fn find_project_version_from_settings(
    owner: &str,
    repo_name: &str,
    tree: &[GitTreeEntry],
) -> Option<String> {
    let settings_paths = ["settings.gradle.kts", "settings.gradle"];
    for path in settings_paths {
        if tree
            .iter()
            .any(|e| e.path == path && e.entry_type == "blob")
            && let Ok(content) = client().get_file_content(owner, repo_name, path)
                && let Some(dsl) = parse_build_gradle_kts(&content)
                    && dsl.project_version.is_some() {
                        return dsl.project_version;
                    }
    }
    None
}

fn tree_has_file(tree: &[GitTreeEntry], path: &str) -> bool {
    tree.iter()
        .any(|e| e.entry_type == "blob" && e.path == path)
}

const LOGO_FILENAMES: &[&str] = &[
    "logo.png",
    "icon.png",
    "logo.jpg",
    "icon.jpg",
    "logo.svg",
    "icon.svg",
    "logo.webp",
    "icon.webp",
];

const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "svg", "webp", "gif"];

pub fn to_raw_url(owner: &str, repo: &str, branch: &str, path: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        owner, repo, branch, path
    )
}

fn find_file_anywhere(tree: &[GitTreeEntry], filename: &str) -> Option<String> {
    tree.iter()
        .filter(|e| e.entry_type == "blob")
        .find(|e| e.path == filename || e.path.ends_with(&format!("/{}", filename)))
        .map(|e| e.path.clone())
}

fn find_logo_url(tree: &[GitTreeEntry], owner: &str, repo: &str, branch: &str) -> Option<String> {
    for filename in LOGO_FILENAMES {
        if let Some(path) = find_file_anywhere(tree, filename) {
            return Some(to_raw_url(owner, repo, branch, &path));
        }
    }
    None
}

fn find_gallery_items(tree: &[GitTreeEntry], owner: &str, repo: &str, branch: &str) -> Vec<GalleryItem> {
    let now = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let mut gallery = Vec::new();
    for i in 1..=10 {
        let mut found = false;
        for ext in IMAGE_EXTENSIONS {
            let filename = format!("gallery{}.{}", i, ext);
            if let Some(path) = find_file_anywhere(tree, &filename) {
                gallery.push(GalleryItem {
                    url: to_raw_url(owner, repo, branch, &path),
                    title: format!("Gallery {}", i),
                    description: String::new(),
                    created: now.clone(),
                });
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
    }
    gallery
}

fn find_first_allay_dsl(
    owner: &str,
    repo_name: &str,
    paths: &[String],
    full_name: &str,
    tree: &[GitTreeEntry],
) -> Option<AllayDsl> {
    let mut settings_version: Option<Option<String>> = None;

    for gradle_path in paths {
        let content = match client().get_file_content(owner, repo_name, gradle_path) {
            Ok(c) => c,
            Err(e) => {
                debug!(repo = %full_name, path = %gradle_path, error = %e, "Skip: failed to get gradle file");
                continue;
            }
        };

        if !content.contains("org.allaymc") {
            debug!(repo = %full_name, path = %gradle_path, "Skip: no org.allaymc dependency");
            continue;
        }

        let mut dsl = match parse_build_gradle_kts(&content) {
            Some(d) => d,
            None => {
                debug!(repo = %full_name, path = %gradle_path, "Skip: failed to parse gradle file");
                continue;
            }
        };

        if dsl.project_version.is_none() {
            let version = settings_version.get_or_insert_with(|| {
                find_project_version_from_settings(owner, repo_name, tree)
            });
            dsl.project_version = version.clone();
        }

        if dsl.plugin.is_none() && dsl.has_allay_dependency
            && let Some(module) = gradle_path_to_module(gradle_path) {
                for json_path in plugin_json_paths_for_module(&module) {
                    if !tree_has_file(tree, &json_path) {
                        continue;
                    }
                    if let Ok(json_content) =
                        client().get_file_content(owner, repo_name, &json_path)
                        && let Some(json) = parse_plugin_json(&json_content)
                            && json.entrance.is_some() {
                                dsl.plugin = Some(json.into_plugin_dsl(
                                    dsl.project_version.as_deref(),
                                    dsl.project_description.as_deref(),
                                ));
                                break;
                            }
                }
            }

        if dsl.plugin.is_some() || dsl.has_allay_dependency {
            return Some(dsl);
        }
    }

    None
}

pub fn build_plugins_from_repo(repo: &Repository, gradle_paths: &[String]) -> Vec<Plugin> {
    let (owner, repo_name) = match repo.full_name.split_once('/') {
        Some((o, r)) => (o, r),
        None => {
            debug!(repo = %repo.full_name, "Skip: invalid repo name");
            return Vec::new();
        }
    };

    let default_branch = repo.default_branch.as_deref().unwrap_or("main");

    let tree = get_tree(owner, repo_name, default_branch);

    let paths_to_check = if gradle_paths.is_empty() {
        find_gradle_paths_from_tree(&tree)
    } else {
        gradle_paths.to_vec()
    };

    let mut dsl =
        match find_first_allay_dsl(owner, repo_name, &paths_to_check, &repo.full_name, &tree) {
            Some(d) => d,
            None => {
                debug!(repo = %repo.full_name, "Skip: no valid gradle modules found");
                return Vec::new();
            }
        };

    resolve_dsl_versions(&mut dsl, &tree, owner, repo_name);

    let releases = client().get_releases(owner, repo_name).unwrap_or_default();
    let readme = client().get_readme(owner, repo_name).unwrap_or_default();
    let contributors = client()
        .get_contributors_by_url(&repo.contributors_url)
        .unwrap_or_default();

    let license = repo.license.as_ref().map_or_else(
        || License {
            id: "ARR".to_string(),
            name: "All Rights Reserved".to_string(),
            url: String::new(),
        },
        |l| {
            let spdx_id = &l.spdx_id;
            let is_valid_spdx = !spdx_id.is_empty()
                && spdx_id != "NOASSERTION"
                && !spdx_id.starts_with("LicenseRef");

            let url = if let Some(html_url) = &l.html_url {
                html_url.clone()
            } else if is_valid_spdx {
                format!("https://spdx.org/licenses/{}.html", spdx_id)
            } else {
                let branch = repo.default_branch.as_deref().unwrap_or("main");
                format!("{}/blob/{}/LICENSE", repo.html_url, branch)
            };

            License {
                id: spdx_id.clone(),
                name: l.name.clone(),
                url,
            }
        },
    );

    let icon_url = find_logo_url(&tree, owner, repo_name, default_branch)
        .unwrap_or_else(|| repo.owner.avatar_url.clone());
    let repo_gallery = find_gallery_items(&tree, owner, repo_name, default_branch);

    let input = PluginBuildInput {
        repo,
        dsl: &dsl,
        releases: &releases,
        readme: &readme,
        license: &license,
        contributors: &contributors,
        owner,
        repo_name,
        branch: default_branch,
        icon_url: &icon_url,
        repo_gallery,
    };

    match build_plugin_from_repo_data(input) {
        Some(plugin) => vec![plugin],
        None => Vec::new(),
    }
}

fn build_plugin_from_repo_data(input: PluginBuildInput) -> Option<Plugin> {
    let PluginBuildInput {
        repo,
        dsl,
        releases,
        readme,
        license,
        contributors,
        owner,
        repo_name,
        branch,
        icon_url,
        repo_gallery,
    } = input;
    let plugin_dsl = match dsl.plugin.as_ref() {
        Some(p) => p,
        None => {
            debug!(repo = %repo.full_name, "Skip: no plugin DSL");
            return None;
        }
    };

    let plugin_name = plugin_dsl.name.clone().unwrap_or_else(|| repo.name.clone());

    let plugin_id = format!("{}/{}", owner, plugin_name).to_lowercase();

    let versions: Vec<Version> = releases
        .iter()
        .filter(|r| !r.draft)
        .map(build_version)
        .filter(|v| !v.files.is_empty())
        .collect();

    let total_downloads: u64 = versions.iter().map(|v| v.downloads).sum();

    let summary = plugin_dsl
        .description
        .clone()
        .or_else(|| repo.description.clone())
        .unwrap_or_default();

    let website = plugin_dsl.website.clone().unwrap_or_default();

    let authors = build_authors(plugin_dsl, repo, contributors);

    let ctx = PostProcessContext {
        owner,
        repo: repo_name,
        branch,
    };
    let (processed_readme, readme_gallery) = process_readme(readme, &ctx);

    let mut gallery = repo_gallery;
    let existing_urls: std::collections::HashSet<String> =
        gallery.iter().map(|g| g.url.clone()).collect();
    for item in readme_gallery {
        if !existing_urls.contains(&item.url) {
            gallery.push(item);
        }
    }

    let api_version = dsl
        .api
        .clone()
        .or_else(|| plugin_dsl.api_version.clone())
        .unwrap_or_default();

    let server_version = dsl.server.clone().unwrap_or_default();

    let dependencies: Vec<Dependency> = plugin_dsl
        .dependencies
        .iter()
        .map(|d| Dependency {
            plugin_id: d.name.to_lowercase(),
            version_range: d.version.clone().unwrap_or_default(),
            dependency_type: if d.optional { "optional" } else { "required" }.to_string(),
        })
        .collect();

    Some(Plugin {
        id: plugin_id,
        name: plugin_name,
        source: repo.html_url.clone(),
        summary,
        description: processed_readme,
        authors,
        categories: build_categories(&repo.topics),
        license: license.clone(),
        links: Some(Links {
            homepage: website,
            wiki: String::new(),
            discord: String::new(),
        }),
        downloads: total_downloads,
        stars: repo.stargazers_count,
        created_at: parse_timestamp(&repo.created_at),
        updated_at: parse_timestamp(&repo.updated_at),
        icon_url: icon_url.to_string(),
        gallery,
        versions,
        api_version,
        server_version,
        dependencies,
        preserved_fields: Default::default(),
    })
}

fn build_version(release: &Release) -> Version {
    let jars: Vec<_> = release
        .assets
        .iter()
        .filter(|a| a.name.ends_with(".jar") || a.name.ends_with(".zip"))
        .collect();

    let primary_jar = jars
        .iter()
        .find(|a| a.name.to_lowercase().contains("allay"))
        .or_else(|| jars.first())
        .map(|a| a.name.as_str());

    let files: Vec<VersionFile> = jars
        .iter()
        .map(|a| VersionFile {
            filename: a.name.clone(),
            url: a.browser_download_url.clone(),
            size: a.size,
            primary: primary_jar == Some(a.name.as_str()),
        })
        .collect();

    let total_downloads: u64 = jars.iter().map(|a| a.download_count).sum();

    Version {
        version: normalize_version(&release.tag_name),
        name: release.name.clone().unwrap_or_else(|| release.tag_name.clone()),
        prerelease: release.prerelease,
        changelog: release.body.clone().unwrap_or_default(),
        files,
        downloads: total_downloads,
        published_at: parse_timestamp(&release.published_at),
    }
}

fn build_categories(topics: &[String]) -> Vec<String> {
    let valid_ids: std::collections::HashSet<&str> = CATEGORIES.iter().copied().collect();
    let categories: Vec<String> = topics
        .iter()
        .filter(|t| valid_ids.contains(t.as_str()))
        .cloned()
        .collect();
    if categories.is_empty() {
        vec!["utility".to_string()]
    } else {
        categories
    }
}

fn build_authors(
    plugin_dsl: &crate::gradle::PluginDsl,
    repo: &Repository,
    contributors: &[Contributor],
) -> Vec<Author> {
    let mut authors = vec![Author {
        name: repo.owner.login.clone(),
        url: repo.owner.html_url.clone(),
        avatar_url: repo.owner.avatar_url.clone(),
    }];

    let owner_lower = repo.owner.login.to_lowercase();
    for name in &plugin_dsl.authors {
        let name_lower = name.to_lowercase();
        if name_lower == owner_lower {
            continue;
        }

        if let Some(contributor) = contributors
            .iter()
            .find(|c| c.login.to_lowercase() == name_lower)
        {
            authors.push(Author {
                name: contributor.login.clone(),
                url: contributor.html_url.clone(),
                avatar_url: contributor.avatar_url.clone(),
            });
        }
    }

    authors
}

fn normalize_version(tag: &str) -> String {
    tag.trim_start_matches('v').to_string()
}

pub fn parse_timestamp(s: &str) -> u64 {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.timestamp() as u64)
        .unwrap_or(0)
}

pub fn parse_github_url(url: &str) -> Option<(String, String)> {
    let url = url.trim_end_matches('/');
    let parts: Vec<&str> = url.split('/').collect();

    if parts.len() >= 2 {
        let repo = parts[parts.len() - 1].to_string();
        let owner = parts[parts.len() - 2].to_string();
        if !owner.is_empty() && !repo.is_empty() {
            return Some((owner, repo));
        }
    }
    None
}
