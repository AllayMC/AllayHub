use super::auth::GitHubAppAuth;
use super::types::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use tracing::{debug, debug_span, info_span, warn};

const MAX_CONCURRENT: usize = 10;

const API_BASE: &str = "https://api.github.com";
const TOKEN_REFRESH_MARGIN: Duration = Duration::from_secs(3600 - 300);
const RATE_LIMIT_BUFFER: usize = 5;

#[derive(Clone)]
pub enum AuthMethod {
    Token(String),
    App(GitHubAppAuth),
    None,
}

pub struct RateLimit {
    remaining: AtomicUsize,
    limit: AtomicUsize,
    reset: AtomicUsize,
}

impl RateLimit {
    fn new() -> Self {
        Self {
            remaining: AtomicUsize::new(usize::MAX),
            limit: AtomicUsize::new(0),
            reset: AtomicUsize::new(0),
        }
    }

    pub fn remaining(&self) -> usize {
        self.remaining.load(Ordering::SeqCst)
    }

    pub fn limit(&self) -> usize {
        self.limit.load(Ordering::SeqCst)
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.load(Ordering::SeqCst) > RATE_LIMIT_BUFFER
    }
}

impl Clone for RateLimit {
    fn clone(&self) -> Self {
        Self {
            remaining: AtomicUsize::new(self.remaining.load(Ordering::SeqCst)),
            limit: AtomicUsize::new(self.limit.load(Ordering::SeqCst)),
            reset: AtomicUsize::new(self.reset.load(Ordering::SeqCst)),
        }
    }
}

pub struct GitHubClient {
    auth: AuthMethod,
    cached_token: RwLock<Option<(String, Instant)>>,
    pub rate_limit: RateLimit,
    api_calls: AtomicUsize,
}

impl Clone for GitHubClient {
    fn clone(&self) -> Self {
        Self {
            auth: self.auth.clone(),
            cached_token: RwLock::new(self.cached_token.read().unwrap().clone()),
            rate_limit: self.rate_limit.clone(),
            api_calls: AtomicUsize::new(self.api_calls.load(Ordering::SeqCst)),
        }
    }
}

impl GitHubClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            auth: token.map(AuthMethod::Token).unwrap_or(AuthMethod::None),
            cached_token: RwLock::new(None),
            rate_limit: RateLimit::new(),
            api_calls: AtomicUsize::new(0),
        }
    }

    pub fn with_app(app_auth: GitHubAppAuth) -> Self {
        Self {
            auth: AuthMethod::App(app_auth),
            cached_token: RwLock::new(None),
            rate_limit: RateLimit::new(),
            api_calls: AtomicUsize::new(0),
        }
    }

    pub fn api_calls(&self) -> usize {
        self.api_calls.load(Ordering::SeqCst)
    }

    fn get_token(&self) -> Result<Option<String>, String> {
        match &self.auth {
            AuthMethod::Token(t) => Ok(Some(t.clone())),
            AuthMethod::None => Ok(None),
            AuthMethod::App(app) => {
                {
                    let cached = self.cached_token.read().unwrap();
                    if let Some((token, created)) = cached.as_ref() {
                        if created.elapsed() < TOKEN_REFRESH_MARGIN {
                            return Ok(Some(token.clone()));
                        }
                    }
                }
                let token = app.get_token()?;
                let mut cached = self.cached_token.write().unwrap();
                *cached = Some((token.clone(), Instant::now()));
                Ok(Some(token))
            }
        }
    }

    fn update_rate_limit_from_headers(
        &self,
        remaining: Option<&str>,
        limit: Option<&str>,
        reset: Option<&str>,
    ) {
        if let Some(r) = remaining.and_then(|s| s.parse().ok()) {
            self.rate_limit
                .remaining
                .store(r, std::sync::atomic::Ordering::SeqCst);
        }
        if let Some(l) = limit.and_then(|s| s.parse().ok()) {
            self.rate_limit
                .limit
                .store(l, std::sync::atomic::Ordering::SeqCst);
        }
        if let Some(r) = reset.and_then(|s| s.parse().ok()) {
            self.rate_limit
                .reset
                .store(r, std::sync::atomic::Ordering::SeqCst);
        }
    }

    fn request<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, String> {
        let _span = debug_span!("api_request", url = %url).entered();
        let token = self.get_token()?;

        for attempt in 0..3 {
            let mut req = ureq::get(url)
                .header("Accept", "application/vnd.github+json")
                .header("User-Agent", "allayindexer")
                .header("X-GitHub-Api-Version", "2022-11-28");

            if let Some(t) = &token {
                req = req.header("Authorization", &format!("Bearer {}", t));
            }

            self.api_calls.fetch_add(1, Ordering::SeqCst);
            match req.call() {
                Ok(mut resp) => {
                    self.update_rate_limit_from_headers(
                        resp.headers()
                            .get("X-RateLimit-Remaining")
                            .and_then(|h| h.to_str().ok()),
                        resp.headers()
                            .get("X-RateLimit-Limit")
                            .and_then(|h| h.to_str().ok()),
                        resp.headers()
                            .get("X-RateLimit-Reset")
                            .and_then(|h| h.to_str().ok()),
                    );
                    return resp
                        .body_mut()
                        .read_json()
                        .map_err(|e| format!("Parse error: {}", e));
                }
                Err(ureq::Error::StatusCode(code)) if code == 403 || code == 429 => {
                    if attempt < 2 {
                        let wait = 60;
                        warn!(
                            code = code,
                            wait = wait,
                            attempt = attempt + 1,
                            "Rate limited, waiting"
                        );
                        thread::sleep(Duration::from_secs(wait));
                        continue;
                    }
                    return Err(format!("Rate limited after {} attempts", attempt + 1));
                }
                Err(e) => return Err(format!("HTTP error: {}", e)),
            }
        }
        Err("Max retries exceeded".to_string())
    }

    fn request_raw(&self, url: &str) -> Result<String, String> {
        let _span = debug_span!("api_request_raw", url = %url).entered();
        let token = self.get_token()?;

        for attempt in 0..3 {
            let mut req = ureq::get(url)
                .header("Accept", "application/vnd.github.raw+json")
                .header("User-Agent", "allayindexer")
                .header("X-GitHub-Api-Version", "2022-11-28");

            if let Some(t) = &token {
                req = req.header("Authorization", &format!("Bearer {}", t));
            }

            self.api_calls.fetch_add(1, Ordering::SeqCst);
            match req.call() {
                Ok(mut resp) => {
                    self.update_rate_limit_from_headers(
                        resp.headers()
                            .get("X-RateLimit-Remaining")
                            .and_then(|h| h.to_str().ok()),
                        resp.headers()
                            .get("X-RateLimit-Limit")
                            .and_then(|h| h.to_str().ok()),
                        resp.headers()
                            .get("X-RateLimit-Reset")
                            .and_then(|h| h.to_str().ok()),
                    );
                    return resp.body_mut().read_to_string().map_err(|e| e.to_string());
                }
                Err(ureq::Error::StatusCode(404)) => return Err("not found".to_string()),
                Err(ureq::Error::StatusCode(code)) if code == 403 || code == 429 => {
                    if attempt < 2 {
                        let wait = 60;
                        warn!(
                            code = code,
                            wait = wait,
                            attempt = attempt + 1,
                            "Rate limited, waiting"
                        );
                        thread::sleep(Duration::from_secs(wait));
                        continue;
                    }
                    return Err(format!("Rate limited after {} attempts", attempt + 1));
                }
                Err(e) => return Err(format!("HTTP error: {}", e)),
            }
        }
        Err("Max retries exceeded".to_string())
    }

    pub fn get_repository(&self, owner: &str, repo: &str) -> Result<Repository, String> {
        let url = format!("{}/repos/{}/{}", API_BASE, owner, repo);
        self.request(&url)
    }

    pub fn get_latest_release(&self, owner: &str, repo: &str) -> Result<Release, String> {
        let url = format!("{}/repos/{}/{}/releases/latest", API_BASE, owner, repo);
        self.request(&url)
    }

    pub fn get_releases(&self, owner: &str, repo: &str) -> Result<Vec<Release>, String> {
        let url = format!("{}/repos/{}/{}/releases?per_page=30", API_BASE, owner, repo);
        self.request(&url)
    }

    pub fn search_repositories(&self, query: &str, page: u32) -> Result<SearchResult, String> {
        let url = format!(
            "{}/search/repositories?q={}&per_page=100&page={}",
            API_BASE,
            urlencoded(query),
            page
        );
        self.request(&url)
    }

    pub fn search_code(&self, query: &str, page: u32) -> Result<CodeSearchResult, String> {
        let url = format!(
            "{}/search/code?q={}&per_page=100&page={}",
            API_BASE,
            urlencoded(query),
            page
        );
        self.request(&url)
    }

    pub fn get_readme(&self, owner: &str, repo: &str) -> Result<String, String> {
        let url = format!("{}/repos/{}/{}/readme", API_BASE, owner, repo);
        match self.request_raw(&url) {
            Ok(s) => Ok(s),
            Err(e) if e == "not found" => Ok(String::new()),
            Err(e) => Err(e),
        }
    }

    pub fn repository_exists(&self, owner: &str, repo: &str) -> bool {
        self.get_repository(owner, repo).is_ok()
    }

    pub fn get_file_content(&self, owner: &str, repo: &str, path: &str) -> Result<String, String> {
        let url = format!("{}/repos/{}/{}/contents/{}", API_BASE, owner, repo, path);
        self.request_raw(&url)
    }

    pub fn get_file_content_at_ref(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        git_ref: &str,
    ) -> Result<String, String> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}?ref={}",
            API_BASE,
            owner,
            repo,
            path,
            urlencoded(git_ref)
        );
        self.request_raw(&url)
    }

    pub fn list_directory(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Vec<String>, String> {
        let url = format!("{}/repos/{}/{}/contents/{}", API_BASE, owner, repo, path);
        let items: Vec<ContentItem> = self.request(&url)?;
        Ok(items
            .into_iter()
            .filter(|i| i.item_type == "dir")
            .map(|i| i.name)
            .collect())
    }

    pub fn get_tree(&self, owner: &str, repo: &str, branch: &str) -> Result<GitTree, String> {
        let url = format!(
            "{}/repos/{}/{}/git/trees/{}?recursive=1",
            API_BASE, owner, repo, branch
        );
        self.request(&url)
    }

    pub fn get_contributors_by_url(&self, url: &str) -> Result<Vec<Contributor>, String> {
        if url.is_empty() {
            return Ok(Vec::new());
        }
        self.request(url)
    }

    pub fn execute_parallel<T, R, F>(&self, items: Vec<T>, handler: F) -> BatchResult<R>
    where
        T: Send + Clone + 'static,
        R: Send + std::fmt::Debug + 'static,
        F: Fn(T, &GitHubClient) -> R + Send + Sync + 'static,
    {
        let _span = info_span!("execute_parallel", total = items.len()).entered();
        let results: Arc<Mutex<Vec<R>>> = Arc::new(Mutex::new(Vec::new()));
        let client = Arc::new(self.clone());
        let handler = Arc::new(handler);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let processed_count = Arc::new(AtomicUsize::new(0));
        let total = items.len();
        let num_chunks = (total + MAX_CONCURRENT - 1) / MAX_CONCURRENT;

        for (chunk_idx, chunk) in items.chunks(MAX_CONCURRENT).enumerate() {
            if stop_flag.load(Ordering::SeqCst) {
                break;
            }

            let _chunk_span = debug_span!(
                "chunk",
                idx = chunk_idx,
                of = num_chunks,
                size = chunk.len()
            )
            .entered();
            let mut handles = Vec::new();

            for item in chunk {
                if stop_flag.load(Ordering::SeqCst) {
                    break;
                }

                let item = item.clone();
                let client = Arc::clone(&client);
                let handler = Arc::clone(&handler);
                let results = Arc::clone(&results);
                let stop_flag = Arc::clone(&stop_flag);
                let processed_count = Arc::clone(&processed_count);

                let handle = thread::spawn(move || {
                    if stop_flag.load(Ordering::SeqCst) {
                        return;
                    }

                    if !client.rate_limit.has_remaining() {
                        stop_flag.store(true, Ordering::SeqCst);
                        return;
                    }

                    let result = handler(item, &client);

                    if !client.rate_limit.has_remaining() {
                        stop_flag.store(true, Ordering::SeqCst);
                    }

                    results.lock().unwrap().push(result);
                    processed_count.fetch_add(1, Ordering::SeqCst);
                });
                handles.push(handle);
            }

            for handle in handles {
                let _ = handle.join();
            }

            let current = processed_count.load(Ordering::SeqCst);
            debug!(processed = current, total = total, "Chunk completed");

            if chunk_idx < num_chunks - 1 && !stop_flag.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(1500));
            }
        }

        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
        let processed = results.len();
        let stopped = stop_flag.load(Ordering::SeqCst);

        if stopped {
            warn!(
                processed = processed,
                total = total,
                "Batch stopped by rate limit"
            );
        }

        BatchResult {
            results,
            processed,
            total,
            stopped_by_rate_limit: stopped,
        }
    }
}

pub struct BatchResult<R> {
    pub results: Vec<R>,
    pub processed: usize,
    pub total: usize,
    pub stopped_by_rate_limit: bool,
}

fn urlencoded(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            ':' => result.push_str("%3A"),
            ' ' => result.push_str("%20"),
            _ => {
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}
