use std::sync::LazyLock;

use regex::Regex;

use crate::plugin::GalleryItem;

use super::PostProcessContext;

static IMG_TAG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<img\s+[^>]*src\s*=\s*["']([^"']+)["'][^>]*>"#).unwrap());
static IMG_ALT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"alt\s*=\s*["']([^"']*)["']"#).unwrap());
static MD_IMG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap());

fn is_relative_path(src: &str) -> bool {
    !src.starts_with("http://") && !src.starts_with("https://") && !src.starts_with("data:")
}

fn to_raw_url(src: &str, ctx: &PostProcessContext) -> String {
    let path = src.trim_start_matches("./");
    format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        ctx.owner, ctx.repo, ctx.branch, path
    )
}

pub fn process_html_images(
    content: &str,
    ctx: &PostProcessContext,
    gallery: &mut Vec<GalleryItem>,
) -> String {
    let mut result = content.to_string();

    for cap in IMG_TAG_RE.captures_iter(content) {
        let full_match = cap.get(0).unwrap().as_str();
        let src = cap.get(1).unwrap().as_str();

        if is_relative_path(src) {
            let raw_url = to_raw_url(src, ctx);
            let new_tag = full_match.replace(src, &raw_url);
            result = result.replace(full_match, &new_tag);

            let title = IMG_ALT_RE
                .captures(full_match)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();

            gallery.push(GalleryItem {
                url: raw_url,
                title,
                description: String::new(),
                created: String::new(),
            });
        }
    }

    result
}

pub fn process_md_images(
    content: &str,
    ctx: &PostProcessContext,
    gallery: &mut Vec<GalleryItem>,
) -> String {
    let mut result = content.to_string();

    for cap in MD_IMG_RE.captures_iter(content) {
        let full_match = cap.get(0).unwrap().as_str();
        let alt = cap.get(1).unwrap().as_str();
        let src = cap.get(2).unwrap().as_str();

        if is_relative_path(src) {
            let raw_url = to_raw_url(src, ctx);
            let new_md = format!("![{}]({})", alt, raw_url);
            result = result.replace(full_match, &new_md);

            gallery.push(GalleryItem {
                url: raw_url,
                title: alt.to_string(),
                description: String::new(),
                created: String::new(),
            });
        }
    }

    result
}
