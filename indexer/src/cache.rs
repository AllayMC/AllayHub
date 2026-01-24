use crate::github::{GitTree, Repository};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use tracing::info;

const CACHE_FILE: &str = ".data_cache.bin.gz";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub etag: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataCache {
    pub repositories: HashMap<String, CacheEntry<Repository>>,
    pub trees: HashMap<String, CacheEntry<GitTree>>,
}

impl DataCache {
    pub fn load() -> Self {
        let file = match File::open(CACHE_FILE) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };

        let mut decoder = GzDecoder::new(BufReader::new(file));
        let mut bytes = Vec::new();
        if decoder.read_to_end(&mut bytes).is_err() {
            return Self::default();
        }

        match postcard::from_bytes(&bytes) {
            Ok(cache) => {
                let cache: DataCache = cache;
                let count = cache.repositories.len() + cache.trees.len();
                if count > 0 {
                    info!(entries = count, "Loaded data cache");
                }
                cache
            }
            Err(e) => {
                info!(error = %e, "Failed to load cache, starting fresh");
                Self::default()
            }
        }
    }

    pub fn save(&self) {
        let count = self.repositories.len() + self.trees.len();
        if count == 0 {
            return;
        }

        let bytes = match postcard::to_allocvec(self) {
            Ok(b) => b,
            Err(e) => {
                info!(error = %e, "Failed to serialize cache");
                return;
            }
        };

        let file = match File::create(CACHE_FILE) {
            Ok(f) => f,
            Err(e) => {
                info!(error = %e, "Failed to create cache file");
                return;
            }
        };

        let mut encoder = GzEncoder::new(file, Compression::default());
        match encoder.write_all(&bytes) {
            Ok(_) => info!(entries = count, "Saved data cache"),
            Err(e) => info!(error = %e, "Failed to write cache"),
        }
    }
}
