use std::collections::HashSet;
use std::fs;

pub fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

pub fn get_arg(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}

pub fn extract_repo_full_name(url: &str) -> Option<String> {
    let url = url.trim_end_matches('/');
    if let Some(rest) = url.strip_prefix("https://github.com/") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 2 {
            return Some(format!("{}/{}", parts[0], parts[1]));
        }
    }
    None
}

pub fn read_last_sync() -> Option<String> {
    fs::read_to_string(".last_sync")
        .ok()
        .map(|s| s.trim().to_string())
}

pub fn write_last_sync() {
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let _ = fs::write(".last_sync", &date);
}

pub fn read_processed_ids() -> HashSet<String> {
    fs::read_to_string(".update_progress")
        .ok()
        .map(|s| s.lines().map(|l| l.to_string()).collect())
        .unwrap_or_default()
}

pub fn write_processed_ids(ids: &HashSet<String>) {
    let content: Vec<_> = ids.iter().map(|s| s.as_str()).collect();
    let _ = fs::write(".update_progress", content.join("\n"));
}

pub fn clear_processed_ids() {
    let _ = fs::remove_file(".update_progress");
}
