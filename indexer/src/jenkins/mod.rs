use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;
use tracing::{debug, info, warn};

const JENKINS_API_URL: &str =
    "https://motci.cn/api/json?tree=jobs[name,url,color,lastSuccessfulBuild[number,timestamp,artifacts[fileName,relativePath],actions[_class,remoteUrls]],jobs[name,url,color,lastSuccessfulBuild[number,timestamp,artifacts[fileName,relativePath],actions[_class,remoteUrls]]]]";

// Jenkins API response types

#[derive(Deserialize)]
struct JenkinsResponse {
    #[serde(default)]
    jobs: Vec<JenkinsJob>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JenkinsJob {
    #[serde(default)]
    name: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    last_successful_build: Option<JenkinsBuild>,
    // Folder jobs contain nested jobs
    #[serde(default)]
    jobs: Option<Vec<JenkinsJob>>,
}

#[derive(Deserialize)]
struct JenkinsBuild {
    number: u64,
    #[serde(default)]
    timestamp: u64,
    #[serde(default)]
    artifacts: Vec<JenkinsArtifact>,
    #[serde(default)]
    actions: Vec<JenkinsAction>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JenkinsArtifact {
    #[serde(default)]
    file_name: String,
    #[serde(default)]
    relative_path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JenkinsAction {
    #[serde(rename = "_class", default)]
    class: String,
    #[serde(default)]
    remote_urls: Option<Vec<String>>,
}

// Parsed build info

pub struct JenkinsBuildInfo {
    pub job_url: String,
    pub build_number: u64,
    pub timestamp: u64,
    pub artifacts: Vec<(String, String)>, // (filename, relative_path)
}

pub struct JenkinsIndex {
    builds: HashMap<String, JenkinsBuildInfo>, // repo_full_name -> build info
}

impl JenkinsIndex {
    pub fn get(&self, repo_full_name: &str) -> Option<&JenkinsBuildInfo> {
        self.builds.get(&repo_full_name.to_lowercase())
    }

    pub fn repo_names(&self) -> impl Iterator<Item = &str> {
        self.builds.keys().map(|s| s.as_str())
    }
}

static INDEX: OnceLock<JenkinsIndex> = OnceLock::new();

pub fn init_jenkins() {
    INDEX.get_or_init(|| match fetch_jenkins_index() {
        Ok(index) => {
            info!(count = index.builds.len(), "Jenkins index loaded");
            index
        }
        Err(e) => {
            warn!(error = %e, "Failed to load Jenkins index, continuing without it");
            JenkinsIndex {
                builds: HashMap::new(),
            }
        }
    });
}

pub fn jenkins_index() -> &'static JenkinsIndex {
    INDEX.get_or_init(|| {
        warn!("Jenkins index accessed before init, returning empty");
        JenkinsIndex {
            builds: HashMap::new(),
        }
    })
}

fn fetch_jenkins_index() -> Result<JenkinsIndex, String> {
    info!("Fetching Jenkins build data from motci.cn");

    let response: JenkinsResponse = ureq::get(JENKINS_API_URL)
        .header("Accept", "application/json")
        .call()
        .map_err(|e| format!("HTTP request failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    let mut builds = HashMap::new();

    for job in &response.jobs {
        process_job(job, &mut builds);
    }

    Ok(JenkinsIndex { builds })
}

fn process_job(job: &JenkinsJob, builds: &mut HashMap<String, JenkinsBuildInfo>) {
    // Folder job: pick "master" sub-job, else first with a successful build
    if let Some(sub_jobs) = &job.jobs {
        let chosen = sub_jobs
            .iter()
            .find(|j| j.name == "master" && j.last_successful_build.is_some())
            .or_else(|| sub_jobs.iter().find(|j| j.last_successful_build.is_some()));

        if let Some(sub) = chosen {
            process_leaf_job(sub, builds);
        }
        return;
    }

    process_leaf_job(job, builds);
}

fn process_leaf_job(job: &JenkinsJob, builds: &mut HashMap<String, JenkinsBuildInfo>) {
    let build = match &job.last_successful_build {
        Some(b) => b,
        None => return,
    };

    let repo_key = match extract_repo_from_build(build) {
        Some(key) => key,
        None => {
            debug!(job = %job.name, "No Git SCM URL found");
            return;
        }
    };

    let artifacts: Vec<(String, String)> = build
        .artifacts
        .iter()
        .map(|a| (a.file_name.clone(), a.relative_path.clone()))
        .collect();

    if artifacts.is_empty() {
        debug!(job = %job.name, "No artifacts");
        return;
    }

    debug!(repo = %repo_key, job = %job.name, build = build.number, "Indexed Jenkins build");

    builds.insert(
        repo_key,
        JenkinsBuildInfo {
            job_url: job.url.clone(),
            build_number: build.number,
            timestamp: build.timestamp,
            artifacts,
        },
    );
}

fn extract_repo_from_build(build: &JenkinsBuild) -> Option<String> {
    for action in &build.actions {
        if action.class != "hudson.plugins.git.util.BuildData" {
            continue;
        }
        if let Some(urls) = &action.remote_urls {
            if let Some(url) = urls.first() {
                return normalize_scm_url(url);
            }
        }
    }
    None
}

fn normalize_scm_url(url: &str) -> Option<String> {
    let url = url.trim().trim_end_matches('/');
    let url = url.strip_suffix(".git").unwrap_or(url);

    // https://github.com/owner/repo or http://...
    if let Some(rest) = url
        .strip_prefix("https://github.com/")
        .or_else(|| url.strip_prefix("http://github.com/"))
    {
        let parts: Vec<&str> = rest.splitn(3, '/').collect();
        if parts.len() >= 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            return Some(format!("{}/{}", parts[0], parts[1]).to_lowercase());
        }
    }

    // git@github.com:owner/repo
    if let Some(rest) = url.strip_prefix("git@github.com:") {
        let parts: Vec<&str> = rest.splitn(3, '/').collect();
        if parts.len() >= 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            return Some(format!("{}/{}", parts[0], parts[1]).to_lowercase());
        }
    }

    None
}
