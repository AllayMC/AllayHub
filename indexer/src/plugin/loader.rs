use super::types::Plugin;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{error, warn};

pub fn load_plugins(index_dir: &Path) -> Vec<Plugin> {
    let mut plugins = Vec::new();

    let entries = match fs::read_dir(index_dir) {
        Ok(e) => e,
        Err(e) => {
            error!(path = ?index_dir, error = %e, "Failed to read directory");
            return plugins;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            if let Ok(content) = fs::read_to_string(&path) {
                match parse_plugin_with_preserved_fields(&content) {
                    Ok(plugin) => plugins.push(plugin),
                    Err(e) => warn!(path = ?path, error = %e, "Failed to parse plugin"),
                }
            }
        }
    }

    plugins
}

pub fn load_plugin(path: &Path) -> Result<Plugin, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    parse_plugin_with_preserved_fields(&content)
}

fn parse_plugin_with_preserved_fields(content: &str) -> Result<Plugin, String> {
    let value: serde_json::Value = serde_json::from_str(content).map_err(|e| e.to_string())?;
    let obj = value.as_object().ok_or("Expected JSON object")?;

    let mut preserved_fields = HashMap::new();
    let mut normalized = serde_json::Map::new();

    for (key, val) in obj {
        if let Some(field_name) = key.strip_prefix('!') {
            preserved_fields.insert(field_name.to_string(), val.clone());
            normalized.insert(field_name.to_string(), val.clone());
        } else {
            normalized.insert(key.clone(), val.clone());
        }
    }

    let mut plugin: Plugin = serde_json::from_value(serde_json::Value::Object(normalized))
        .map_err(|e| e.to_string())?;
    plugin.preserved_fields = preserved_fields;
    Ok(plugin)
}
