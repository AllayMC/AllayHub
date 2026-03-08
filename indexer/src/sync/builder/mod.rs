mod image;
mod link;

use crate::github::{client, Contributor, GitTreeEntry, Release, Repository};
use crate::plugin::{
    Author, Dependency, GalleryItem, License, Links, Plugin, Version, VersionFile,
};
use tracing::debug;

fn parse_timestamp(iso_string: &str) -> u64 {
    use chrono::{DateTime, Utc};
    DateTime::parse_from_rfc3339(iso_string)
        .ok()
        .map(|dt| dt.with_timezone(&Utc).timestamp() as u64)
        .unwrap_or(0)
}

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

pub struct PostProcessContext<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub branch: &'a str,
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

fn get_tree(owner: &str, repo: &str, branch: &str) -> Vec<GitTreeEntry> {
    client()
        .get_tree(owner, repo, branch)
        .map(|t| t.tree)
        .unwrap_or_default()
}

fn find_logo_url(tree: &[GitTreeEntry], owner: &str, repo: &str, branch: &str) -> Option<String> {
    let logo_paths = [
        ".github/img/logo.png",
        ".github/img/icon.png",
        "logo.png",
        "icon.png",
    ];

    for path in &logo_paths {
        if tree.iter().any(|e| e.path == *path) {
            return Some(format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, branch, path
            ));
        }
    }
    None
}

fn to_raw_url(owner: &str, repo: &str, branch: &str, path: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        owner, repo, branch, path
    )
}

fn find_gallery_items(
    tree: &[GitTreeEntry],
    owner: &str,
    repo: &str,
    branch: &str,
) -> Vec<GalleryItem> {
    let gallery_dir = ".github/img/";
    let excluded = ["logo.png", "icon.png"];

    tree.iter()
        .filter(|e| {
            e.entry_type == "blob"
                && e.path.starts_with(gallery_dir)
                && e.path.ends_with(".png")
                && !excluded.iter().any(|ex| e.path.ends_with(ex))
        })
        .map(|e| GalleryItem {
            url: format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, branch, e.path
            ),
            title: String::new(),
            description: String::new(),
            created: String::new(),
        })
        .collect()
}

/// Checks if the repository targets Nukkit/NukkitX or Nukkit-MOT.
/// Excludes incompatible forks like PowerNukkitX and Lumi.
fn is_nukkit_target(
    tree: &[GitTreeEntry],
    owner: &str,
    repo_name: &str,
    plugin_yml_path: &str,
) -> bool {
    // If the repo has a build on motci.cn, confirm as NKX/NKMOT
    let repo_key = format!("{}/{}", owner, repo_name);
    if crate::jenkins::jenkins_index().get(&repo_key).is_some() {
        debug!(repo = %repo_key, "NKX/NKMOT confirmed via motci.cn");
        return true;
    }

    let build_paths = find_build_file_paths(tree, plugin_yml_path);

    if build_paths.is_empty() {
        debug!(
            repo = %format!("{}/{}", owner, repo_name),
            "No build files found, skipping"
        );
        return false;
    }

    for path in &build_paths {
        match client().get_file_content(owner, repo_name, path) {
            Ok(content) => {
                let lower = content.to_lowercase();

                if let Some(excluded_target) = detect_excluded_target(&lower) {
                    debug!(
                        repo = %format!("{}/{}", owner, repo_name),
                        file = %path,
                        target = excluded_target,
                        "Skip: excluded server target found"
                    );
                    return false;
                }

                if has_nukkit_indicators(&lower) {
                    debug!(
                        repo = %format!("{}/{}", owner, repo_name),
                        file = %path,
                        "NKX/NKMOT dependency confirmed"
                    );
                    return true;
                }
            }
            Err(e) => {
                debug!(
                    repo = %format!("{}/{}", owner, repo_name),
                    file = %path,
                    error = %e,
                    "Failed to read build file"
                );
            }
        }
    }

    debug!(
        repo = %format!("{}/{}", owner, repo_name),
        "No NKX/NKMOT dependency found"
    );
    false
}

fn find_build_file_paths<'a>(tree: &'a [GitTreeEntry], plugin_yml_path: &str) -> Vec<&'a str> {
    let module_prefix = plugin_yml_path
        .find("src/main/resources")
        .map(|pos| &plugin_yml_path[..pos])
        .unwrap_or("");

    let build_names = ["build.gradle.kts", "build.gradle", "pom.xml"];

    let mut expected: Vec<String> = Vec::new();

    // Module-level build files (for multi-module projects)
    if !module_prefix.is_empty() {
        for name in &build_names {
            expected.push(format!("{}{}", module_prefix, name));
        }
    }

    // Root-level build files
    for name in &build_names {
        expected.push(name.to_string());
    }

    // Gradle version catalog
    expected.push("gradle/libs.versions.toml".to_string());

    tree.iter()
        .filter(|e| e.entry_type == "blob" && expected.iter().any(|exp| e.path == *exp))
        .map(|e| e.path.as_str())
        .collect()
}

fn has_nukkit_indicators(content_lower: &str) -> bool {
    const INDICATORS: &[&str] = &[
        "cn.nukkit",        // Nukkit/NukkitX/NKMOT Maven group
        "cloudburstmc",     // NukkitX GitHub org (JitPack)
        "memoriesoftime",   // Nukkit-MOT GitHub org (JitPack)
        "nukkit-mot",       // Nukkit-MOT reference
        "opencollab.dev",   // NukkitX Maven repo
        "repo.nukkitx.com", // NukkitX Maven repo
    ];

    INDICATORS.iter().any(|p| content_lower.contains(p))
}

fn detect_excluded_target(content_lower: &str) -> Option<&'static str> {
    const EXCLUDED_TARGETS: &[(&str, &str)] = &[
        ("powernukkitx", "PowerNukkitX"),
        ("repo.luminiadev.com", "Lumi"),
        ("com.koshakmine:lumi", "Lumi"),
        ("<groupid>com.koshakmine</groupid>", "Lumi"),
        ("<artifactid>lumi</artifactid>", "Lumi"),
    ];

    EXCLUDED_TARGETS
        .iter()
        .find_map(|(indicator, target)| content_lower.contains(indicator).then_some(*target))
}

pub fn build_plugins_from_nukkit(repo: &Repository, plugin_yml_path: &str) -> Vec<Plugin> {
    build_plugins_from_nukkit_with_tree(repo, plugin_yml_path, None)
}

pub fn build_plugins_from_nukkit_with_tree(
    repo: &Repository,
    plugin_yml_path: &str,
    prefetched_tree: Option<Vec<GitTreeEntry>>,
) -> Vec<Plugin> {
    let (owner, repo_name) = match repo.full_name.split_once('/') {
        Some((o, r)) => (o, r),
        None => {
            debug!(repo = %repo.full_name, "Skip: invalid repo name");
            return Vec::new();
        }
    };

    let default_branch = repo.default_branch.as_deref().unwrap_or("main");

    // Get tree early for dependency check
    let tree = prefetched_tree.unwrap_or_else(|| get_tree(owner, repo_name, default_branch));

    // Only collect plugins that target NKX/NKMOT
    if !is_nukkit_target(&tree, owner, repo_name, plugin_yml_path) {
        debug!(repo = %repo.full_name, "Skip: not targeting NKX/NKMOT");
        return Vec::new();
    }

    let yml_content = match client().get_file_content(owner, repo_name, plugin_yml_path) {
        Ok(content) => content,
        Err(e) => {
            debug!(repo = %repo.full_name, error = %e, "Failed to read plugin.yml");
            return Vec::new();
        }
    };

    let nukkit_yml = match crate::nukkit::NukkitPluginYml::from_str(&yml_content) {
        Ok(yml) => yml,
        Err(e) => {
            debug!(repo = %repo.full_name, error = %e, "Failed to parse plugin.yml");
            return Vec::new();
        }
    };

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
                format!("{}/blob/{}/LICENSE", repo.html_url, default_branch)
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

    match nukkit_yml_to_plugin(
        nukkit_yml,
        repo,
        &releases,
        &readme,
        &license,
        &contributors,
        owner,
        repo_name,
        default_branch,
        &icon_url,
        repo_gallery,
    ) {
        Some(plugin) => vec![plugin],
        None => Vec::new(),
    }
}

fn is_placeholder(s: &str) -> bool {
    // Detect unresolved placeholders like ${project.name} or @project.name@
    let trimmed = s.trim();
    (trimmed.starts_with("${") && trimmed.ends_with('}') && trimmed.len() > 3)
        || (trimmed.starts_with('@') && trimmed.ends_with('@') && trimmed.len() > 2)
}

fn nukkit_yml_to_plugin(
    yml: crate::nukkit::NukkitPluginYml,
    repo: &Repository,
    releases: &[Release],
    readme: &str,
    license: &License,
    _contributors: &[Contributor],
    owner: &str,
    repo_name: &str,
    branch: &str,
    icon_url: &str,
    repo_gallery: Vec<GalleryItem>,
) -> Option<Plugin> {
    let ctx = PostProcessContext {
        owner,
        repo: repo_name,
        branch,
    };

    let (processed_readme, mut gallery) = process_readme(readme, &ctx);
    gallery.extend(repo_gallery);

    let authors = yml
        .all_authors()
        .into_iter()
        .map(|name| Author {
            name,
            url: String::new(),
            avatar_url: String::new(),
        })
        .collect();

    let mut all_dependencies: Vec<Dependency> = yml
        .depend
        .iter()
        .map(|name| Dependency {
            plugin_id: name.clone(),
            version_range: String::new(),
            dependency_type: "required".to_string(),
        })
        .collect();

    all_dependencies.extend(yml.softdepend.iter().map(|name| Dependency {
        plugin_id: name.clone(),
        version_range: String::new(),
        dependency_type: "optional".to_string(),
    }));

    // Build versions from releases
    let mut versions: Vec<Version> = releases
        .iter()
        .filter_map(|release| {
            let files: Vec<VersionFile> = release
                .assets
                .iter()
                .filter(|a| a.name.ends_with(".jar"))
                .map(|a| VersionFile {
                    filename: a.name.clone(),
                    url: a.browser_download_url.clone(),
                    size: a.size,
                    primary: true,
                })
                .collect();

            if files.is_empty() {
                debug!(
                    repo = %repo.full_name,
                    release = %release.tag_name,
                    "Skipping release without .jar assets"
                );
                return None;
            }

            Some(Version {
                version: release.tag_name.clone(),
                name: release
                    .name
                    .clone()
                    .unwrap_or_else(|| release.tag_name.clone()),
                prerelease: release.prerelease,
                changelog: release.body.clone().unwrap_or_default(),
                files,
                downloads: 0,
                published_at: parse_timestamp(&release.published_at),
            })
        })
        .collect();

    // Merge motci.cn CI build
    let repo_key = format!("{}/{}", owner, repo_name);
    if let Some(build) = crate::jenkins::jenkins_index().get(&repo_key) {
        let ci_files: Vec<VersionFile> = build
            .artifacts
            .iter()
            .filter(|(name, _)| name.ends_with(".jar"))
            .map(|(name, path)| VersionFile {
                filename: name.clone(),
                url: format!("{}{}/artifact/{}", build.job_url, build.build_number, path),
                size: 0,
                primary: true,
            })
            .collect();
        if !ci_files.is_empty() {
            versions.push(Version {
                version: format!("ci-{}", build.build_number),
                name: format!("CI Build #{}", build.build_number),
                prerelease: true,
                changelog: String::new(),
                files: ci_files,
                downloads: 0,
                published_at: build.timestamp / 1000,
            });
        }
    }

    let api_version = yml
        .api
        .primary()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty() && !is_placeholder(v))
        .unwrap_or_else(|| "unknown".to_string());

    let categories: Vec<String> = repo
        .topics
        .iter()
        .filter_map(|t| {
            // Strip "nukkit-" prefix if present
            let normalized = t.strip_prefix("nukkit-").unwrap_or(t);
            if CATEGORIES.contains(&normalized) {
                Some(normalized.to_string())
            } else {
                None
            }
        })
        .collect();

    // Handle placeholder names by falling back to repo name
    let plugin_name = if is_placeholder(&yml.name) || yml.name.trim().is_empty() {
        debug!(
            repo = %repo.full_name,
            original = %yml.name,
            "Using repo name as fallback for placeholder"
        );
        repo_name.to_string()
    } else {
        yml.name.trim().to_string()
    };

    let summary = match yml.description.as_deref().map(str::trim) {
        Some(text) if !text.is_empty() && !is_placeholder(text) => text.to_string(),
        _ => repo
            .description
            .clone()
            .unwrap_or_default()
            .trim()
            .to_string(),
    };

    Some(Plugin {
        id: format!("{}/{}", owner, repo_name),
        name: plugin_name,
        source: repo.html_url.clone(),
        summary,
        description: processed_readme,
        authors,
        categories,
        license: license.clone(),
        links: yml.website.as_ref().map(|w| Links {
            homepage: w.clone(),
            wiki: String::new(),
            discord: String::new(),
        }),
        downloads: 0,
        stars: repo.stargazers_count,
        created_at: parse_timestamp(&repo.created_at),
        updated_at: parse_timestamp(&repo.updated_at),
        icon_url: icon_url.to_string(),
        gallery,
        versions,
        api_version,
        server_version: String::new(),
        dependencies: all_dependencies,
        preserved_fields: Default::default(),
    })
}

pub fn parse_github_url(url: &str) -> Option<(String, String)> {
    let url = url.trim_end_matches('/');

    if let Some(rest) = url.strip_prefix("https://github.com/") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 2 {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{detect_excluded_target, has_nukkit_indicators};

    #[test]
    fn detects_supported_nukkit_markers() {
        let gradle = r#"
            repositories {
                maven { url = uri("https://repo.opencollab.dev/maven-releases/") }
            }

            dependencies {
                compileOnly("cn.nukkit:nukkit:1.0-SNAPSHOT")
            }
        "#;

        assert!(has_nukkit_indicators(&gradle.to_lowercase()));
    }

    #[test]
    fn excludes_powernukkitx_targets() {
        let gradle = r#"
            dependencies {
                compileOnly("cn.powernukkitx:powernukkitx:1.20.0-r1")
            }
        "#;

        assert_eq!(
            detect_excluded_target(&gradle.to_lowercase()),
            Some("PowerNukkitX")
        );
    }

    #[test]
    fn excludes_lumi_targets() {
        let gradle = r#"
            repositories {
                maven { url = uri("https://repo.luminiadev.com/snapshots") }
            }

            dependencies {
                compileOnly("com.koshakmine:Lumi:1.5.0-SNAPSHOT")
            }
        "#;

        assert_eq!(detect_excluded_target(&gradle.to_lowercase()), Some("Lumi"));
    }

    #[test]
    fn excludes_lumi_targets_in_maven_pom() {
        let pom = r#"
            <dependency>
                <groupId>com.koshakmine</groupId>
                <artifactId>Lumi</artifactId>
                <version>1.5.0-SNAPSHOT</version>
            </dependency>
        "#;

        assert_eq!(detect_excluded_target(&pom.to_lowercase()), Some("Lumi"));
    }
}
