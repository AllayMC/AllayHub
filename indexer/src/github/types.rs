use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repository {
    pub id: u64,
    pub full_name: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub stargazers_count: u64,
    #[serde(default)]
    pub forks_count: u64,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub pushed_at: String,
    pub owner: Owner,
    #[serde(default)]
    pub license: Option<RepositoryLicense>,
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub is_template: bool,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub default_branch: Option<String>,
    #[serde(default)]
    pub contributors_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Owner {
    pub login: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub html_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepositoryLicense {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub spdx_id: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Release {
    pub id: u64,
    pub tag_name: String,
    pub name: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub published_at: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseAsset {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub download_count: u64,
    #[serde(default)]
    pub browser_download_url: String,
    #[serde(default)]
    pub content_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResult {
    pub total_count: u64,
    pub incomplete_results: bool,
    pub items: Vec<Repository>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReadmeContent {
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub encoding: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeSearchResult {
    pub total_count: u64,
    pub incomplete_results: bool,
    pub items: Vec<CodeSearchItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeSearchItem {
    pub name: String,
    pub path: String,
    pub repository: CodeSearchRepository,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeSearchRepository {
    pub id: u64,
    pub full_name: String,
    #[serde(default)]
    pub fork: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentItem {
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitTree {
    pub sha: String,
    pub tree: Vec<GitTreeEntry>,
    #[serde(default)]
    pub truncated: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitTreeEntry {
    pub path: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    #[serde(default)]
    pub sha: String,
    #[serde(default)]
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contributor {
    pub login: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub contributions: u64,
}
