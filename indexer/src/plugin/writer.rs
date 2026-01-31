use super::types::Plugin;
use std::fs;
use std::path::Path;

pub fn write_plugin(plugin: &Plugin, output_dir: &Path) -> Result<(), String> {
    let path = plugin_path(output_dir, &plugin.id);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create dir {:?}: {}", parent, e))?;
    }

    let mut value = serde_json::to_value(plugin).map_err(|e| e.to_string())?;

    if let Some(obj) = value.as_object_mut() {
        for key in plugin.preserved_fields.keys() {
            if let Some(val) = obj.remove(key) {
                obj.insert(format!("!{}", key), val);
            }
        }
    }

    let json = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| format!("Failed to write {:?}: {}", path, e))
}

pub fn delete_plugin(plugin_id: &str, output_dir: &Path) -> Result<(), String> {
    let path = plugin_path(output_dir, plugin_id);
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete {:?}: {}", path, e))?;
        // Clean up empty owner directory
        if let Some(parent) = path.parent() {
            if parent != output_dir {
                let _ = fs::remove_dir(parent); // ignore error if not empty
            }
        }
        Ok(())
    } else {
        Ok(())
    }
}

fn plugin_path(output_dir: &Path, plugin_id: &str) -> std::path::PathBuf {
    output_dir.join(format!("{}.json", plugin_id))
}
