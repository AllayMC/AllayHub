use std::sync::LazyLock;

use regex::Regex;

use super::PostProcessContext;

static MD_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap());

static HTML_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<a\s+[^>]*href\s*=\s*["']([^"']+)["'][^>]*>"#).unwrap());

fn is_relative_path(src: &str) -> bool {
    !src.starts_with("http://")
        && !src.starts_with("https://")
        && !src.starts_with("data:")
        && !src.starts_with('#')
        && !src.starts_with("mailto:")
}

fn to_blob_url(src: &str, ctx: &PostProcessContext) -> String {
    let path = src.trim_start_matches("./");
    format!(
        "https://github.com/{}/{}/blob/{}/{}",
        ctx.owner, ctx.repo, ctx.branch, path
    )
}

pub fn process_md_links(content: &str, ctx: &PostProcessContext) -> String {
    let mut result = content.to_string();

    for cap in MD_LINK_RE.captures_iter(content) {
        let full_match = cap.get(0).unwrap();
        let start = full_match.start();

        if start > 0 && content.as_bytes()[start - 1] == b'!' {
            continue;
        }

        let text = cap.get(1).unwrap().as_str();
        let href = cap.get(2).unwrap().as_str();

        if is_relative_path(href) {
            let blob_url = to_blob_url(href, ctx);
            let new_md = format!("[{}]({})", text, blob_url);
            result = result.replace(full_match.as_str(), &new_md);
        }
    }

    result
}

pub fn process_html_links(content: &str, ctx: &PostProcessContext) -> String {
    let mut result = content.to_string();

    for cap in HTML_LINK_RE.captures_iter(content) {
        let full_match = cap.get(0).unwrap().as_str();
        let href = cap.get(1).unwrap().as_str();

        if is_relative_path(href) {
            let blob_url = to_blob_url(href, ctx);
            let new_tag = full_match.replace(href, &blob_url);
            result = result.replace(full_match, &new_tag);
        }
    }

    result
}