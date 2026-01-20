use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub authors: Vec<Author>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub license: License,
    #[serde(default)]
    pub links: Option<Links>,
    #[serde(default)]
    pub downloads: u64,
    #[serde(default)]
    pub stars: u64,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub icon_url: String,
    #[serde(default)]
    pub gallery: Vec<GalleryItem>,
    #[serde(default)]
    pub versions: Vec<Version>,
    #[serde(default)]
    pub api_version: String,
    #[serde(default)]
    pub server_version: String,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(default, skip_serializing)]
    pub preserved_fields: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Author {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub avatar_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct License {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Links {
    #[serde(default)]
    pub homepage: String,
    #[serde(default)]
    pub wiki: String,
    #[serde(default)]
    pub discord: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GalleryItem {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub created: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(default)]
    pub changelog: String,
    #[serde(default)]
    pub files: Vec<VersionFile>,
    #[serde(default)]
    pub downloads: u64,
    #[serde(default)]
    pub published_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionFile {
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub primary: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dependency {
    pub plugin_id: String,
    #[serde(default)]
    pub version_range: String,
    #[serde(default)]
    pub dependency_type: String,
}

impl Plugin {
    pub fn get_author_name(&self) -> String {
        self.authors
            .first()
            .map(|a| a.name.clone())
            .unwrap_or_default()
    }
}
