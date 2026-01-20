use crate::github::{GitTreeEntry, client};
use crate::gradle::VersionRef;
use std::sync::OnceLock;
use tracing::debug;

static SNAPSHOT_VERSION_CACHE: OnceLock<Option<String>> = OnceLock::new();

pub fn resolve_version(
    version_ref: &VersionRef,
    tree: &[GitTreeEntry],
    owner: &str,
    repo: &str,
) -> Option<String> {
    let version = match version_ref {
        VersionRef::Literal(v) => Some(v.clone()),
        VersionRef::VersionCatalog(path) => resolve_version_catalog(path, tree, owner, repo),
        VersionRef::Variable(var_path) => resolve_variable(var_path, tree, owner, repo),
        VersionRef::None => None,
    };

    match version {
        Some(v) if is_snapshot_version(&v) => resolve_snapshot_version(),
        other => other,
    }
}

fn is_snapshot_version(version: &str) -> bool {
    let lower = version.to_lowercase();
    lower.ends_with("-snapshot") && !lower.chars().next().map_or(false, |c| c.is_ascii_digit())
}

pub fn resolve_snapshot_version() -> Option<String> {
    SNAPSHOT_VERSION_CACHE
        .get_or_init(|| {
            let releases = client().get_releases("AllayMC", "Allay").ok()?;
            let release = releases.iter().find(|r| !r.prerelease && !r.draft)?;
            if let Some(name) = &release.name {
                if let Some(api_ver) = extract_api_version_from_name(name) {
                    return Some(api_ver);
                }
            }
            None
        })
        .clone()
}

fn extract_api_version_from_name(name: &str) -> Option<String> {
    let api_start = name.find("(API ")?;
    let version_start = api_start + 5;
    let remaining = &name[version_start..];
    let version_end = remaining.find(')')?;
    let version = remaining[..version_end].trim();
    if !version.is_empty() {
        Some(version.to_string())
    } else {
        None
    }
}

fn resolve_version_catalog(
    _path: &str,
    tree: &[GitTreeEntry],
    owner: &str,
    repo: &str,
) -> Option<String> {
    let toml_path = tree
        .iter()
        .find(|e| {
            e.entry_type == "blob"
                && (e.path == "gradle/libs.versions.toml" || e.path == "libs.versions.toml")
        })
        .map(|e| e.path.as_str())?;

    let content = client().get_file_content(owner, repo, toml_path).ok()?;

    parse_allay_version_from_toml(&content)
}

fn parse_allay_version_from_toml(content: &str) -> Option<String> {
    let toml: toml::Value = content.parse().ok()?;

    if let Some(libs) = toml.get("libraries") {
        if let Some(allay) = libs.get("allay") {
            if let Some(version) = allay.get("version") {
                if let Some(v) = version.as_str() {
                    return Some(v.to_string());
                }
                if let Some(ref_name) = version.get("ref").and_then(|r| r.as_str()) {
                    if let Some(versions) = toml.get("versions") {
                        if let Some(v) = versions.get(ref_name).and_then(|v| v.as_str()) {
                            return Some(v.to_string());
                        }
                    }
                }
            }
        }
    }

    if let Some(versions) = toml.get("versions") {
        if let Some(v) = versions.get("allay").and_then(|v| v.as_str()) {
            return Some(v.to_string());
        }
        if let Some(v) = versions.get("allay-api").and_then(|v| v.as_str()) {
            return Some(v.to_string());
        }
    }

    None
}

fn resolve_variable(
    var_path: &str,
    tree: &[GitTreeEntry],
    owner: &str,
    repo: &str,
) -> Option<String> {
    let first_part = var_path.split('.').next()?;
    let search_name = first_part.to_lowercase();

    let candidates: Vec<&str> = tree
        .iter()
        .filter(|e| {
            if e.entry_type != "blob" {
                return false;
            }
            let path_lower = e.path.to_lowercase();
            let filename = e.path.rsplit('/').next().unwrap_or(&e.path);
            let filename_lower = filename.to_lowercase();

            (filename_lower.starts_with(&search_name)
                || path_lower.contains("buildsrc")
                || path_lower.contains("buildlogic"))
                && (filename.ends_with(".kt") || filename.ends_with(".java"))
        })
        .map(|e| e.path.as_str())
        .collect();

    for candidate in candidates {
        if let Ok(content) = client().get_file_content(owner, repo, candidate) {
            if let Some(version) = extract_allay_version_from_code(&content, var_path) {
                debug!(file = candidate, version = %version, "Resolved variable version");
                return Some(version);
            }
        }
    }

    None
}

fn extract_allay_version_from_code(content: &str, var_path: &str) -> Option<String> {
    let parts: Vec<&str> = var_path.split('.').collect();
    let last_part = parts.last()?;

    for line in content.lines() {
        let line = line.trim();

        if !line.contains(last_part) {
            continue;
        }

        if let Some(version) = extract_version_from_line(line) {
            return Some(version);
        }
    }

    for line in content.lines() {
        let line = line.trim();
        let line_lower = line.to_lowercase();

        if line_lower.contains("allay")
            && (line_lower.contains("api") || line_lower.contains("version"))
        {
            if let Some(version) = extract_version_from_line(line) {
                return Some(version);
            }
        }
    }

    None
}

fn extract_version_from_line(line: &str) -> Option<String> {
    if let Some(start) = line.find('"') {
        if let Some(end) = line[start + 1..].find('"') {
            let version = &line[start + 1..start + 1 + end];
            if looks_like_version(version) {
                return Some(version.to_string());
            }
        }
    }

    None
}

fn looks_like_version(s: &str) -> bool {
    if s.is_empty() || s.len() > 50 {
        return false;
    }

    let has_digit = s.chars().any(|c| c.is_ascii_digit());
    let has_dot_or_dash = s.contains('.') || s.contains('-');

    has_digit && has_dot_or_dash
}
