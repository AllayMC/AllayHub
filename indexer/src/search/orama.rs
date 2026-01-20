use crate::plugin::Plugin;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use tracing::{debug, error, info_span};

use super::segment::{get_segmenter, split_identifier};
use instant_segment::Search;

#[derive(Debug, Serialize)]
pub struct OramaDocument {
    pub name: String,
    pub owner: String,
    pub categories: Vec<String>,
    pub api_version: String,
    pub license: String,
    pub downloads: u64,
    pub stars: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub id: String,
    pub display_name: String,
    pub author: String,
    pub summary: String,
    pub icon_url: String,
    pub gallery_url: String,
}

fn get_license_type(license: &crate::plugin::License) -> &'static str {
    if license.id == "ARR" || license.id.is_empty() {
        "closed-source"
    } else {
        "open-source"
    }
}

fn build_document(plugin: &Plugin, split_cache: &HashMap<String, Vec<String>>) -> OramaDocument {
    let author = plugin.get_author_name();

    let name_tokens = split_cache
        .get(&plugin.name)
        .cloned()
        .unwrap_or_else(|| vec![plugin.name.to_lowercase()]);

    let owner_tokens = if author.is_empty() {
        vec![]
    } else {
        split_cache
            .get(&author)
            .cloned()
            .unwrap_or_else(|| vec![author.to_lowercase()])
    };

    OramaDocument {
        name: name_tokens.join(" "),
        owner: owner_tokens.join(" "),
        categories: plugin.categories.clone(),
        api_version: plugin.api_version.clone(),
        license: get_license_type(&plugin.license).to_string(),
        downloads: plugin.downloads,
        stars: plugin.stars,
        created_at: plugin.created_at,
        updated_at: plugin.updated_at,
        id: plugin.id.clone(),
        display_name: plugin.name.clone(),
        author,
        summary: plugin.summary.clone(),
        icon_url: plugin.icon_url.clone(),
        gallery_url: plugin
            .gallery
            .first()
            .map(|g| g.url.clone())
            .unwrap_or_default(),
    }
}

pub fn build_orama_index(plugins: &[Plugin], output_path: &Path, builder_path: &Path) -> bool {
    let _span = info_span!("build_orama_index", plugins = plugins.len()).entered();

    if let Some(parent) = output_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            error!(error = %e, "Failed to create output directory");
            return false;
        }
    }

    let segmenter = get_segmenter();
    let mut identifiers: HashSet<String> = HashSet::new();

    for p in plugins {
        identifiers.insert(p.name.clone());
        let author = p.get_author_name();
        if !author.is_empty() {
            identifiers.insert(author);
        }
    }

    let mut search = Search::default();
    let split_cache: HashMap<String, Vec<String>> = identifiers
        .into_iter()
        .map(|ident| {
            let tokens = split_identifier(&ident, segmenter, &mut search);
            (ident, tokens)
        })
        .collect();

    let docs: Vec<OramaDocument> = plugins
        .iter()
        .map(|p| build_document(p, &split_cache))
        .collect();

    let json = serde_json::to_string(&docs).unwrap_or_default();

    let mut child = match Command::new("bun")
        .args([
            "run",
            &builder_path.to_string_lossy(),
            &output_path.to_string_lossy(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            error!(error = %e, "Failed to spawn bun");
            return false;
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        if let Err(e) = stdin.write_all(json.as_bytes()) {
            error!(error = %e, "Failed to write to stdin");
            return false;
        }
    }

    match child.wait_with_output() {
        Ok(output) => {
            if !output.status.success() {
                error!(stderr = %String::from_utf8_lossy(&output.stderr), "Orama builder failed");
                return false;
            }
            if !output.stdout.is_empty() {
                debug!(stdout = %String::from_utf8_lossy(&output.stdout), "Orama builder output");
            }
            true
        }
        Err(e) => {
            error!(error = %e, "Failed to wait for bun");
            false
        }
    }
}
