use super::types::Plugin;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{error, warn};

pub fn load_plugins(index_dir: &Path) -> Vec<Plugin> {
    let mut plugins = Vec::new();
    load_plugins_recursive(index_dir, &mut plugins);
    plugins
}

fn load_plugins_recursive(dir: &Path, plugins: &mut Vec<Plugin>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            error!(path = ?dir, error = %e, "Failed to read directory");
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            load_plugins_recursive(&path, plugins);
        } else if path.extension().is_some_and(|e| e == "json")
            && let Ok(content) = fs::read_to_string(&path) {
                match parse_plugin_with_preserved_fields(&content) {
                    Ok(plugin) => plugins.push(plugin),
                    Err(e) => warn!(path = ?path, error = %e, "Failed to parse plugin"),
                }
            }
    }
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
