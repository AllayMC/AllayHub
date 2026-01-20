use instant_segment::{Search, Segmenter};
use smartstring::alias::String as SmartString;
use std::collections::HashMap;
use std::sync::OnceLock;

static UNIGRAMS_DATA: &str = include_str!("../../data/en-unigrams.txt");
static SEGMENTER: OnceLock<Segmenter> = OnceLock::new();

pub fn get_segmenter() -> &'static Segmenter {
    SEGMENTER.get_or_init(|| {
        let mut unigrams: HashMap<SmartString, f64> = HashMap::new();

        for line in UNIGRAMS_DATA.lines() {
            if let Some((word, count_str)) = line.split_once('\t') {
                if let Ok(count) = count_str.trim().parse::<f64>() {
                    unigrams.insert(word.into(), count);
                }
            }
        }

        let bigrams: Vec<((SmartString, SmartString), f64)> = Vec::new();
        Segmenter::new(unigrams, bigrams)
    })
}

pub fn pre_split(name: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();

    for ch in name.chars() {
        if ch == '_' || ch == '-' || ch == ' ' || ch == '.' {
            if !current.is_empty() {
                parts.push(current.clone());
                current.clear();
            }
        } else if ch.is_uppercase()
            && !current.is_empty()
            && current
                .chars()
                .last()
                .map(|c| c.is_lowercase())
                .unwrap_or(false)
        {
            parts.push(current.clone());
            current.clear();
            current.push(ch);
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

pub fn split_identifier(name: &str, segmenter: &Segmenter, search: &mut Search) -> Vec<String> {
    if name.is_empty() {
        return vec![];
    }

    let mut tokens = Vec::new();
    let parts = pre_split(name);

    for part in parts {
        let lower = part.to_lowercase();
        if let Ok(words) = segmenter.segment(&lower, search) {
            for word in words {
                if !word.is_empty() {
                    tokens.push(word.to_string());
                }
            }
        } else {
            tokens.push(lower);
        }
    }

    let original = name.to_lowercase();
    if !tokens.contains(&original) {
        tokens.push(original);
    }

    tokens
}
