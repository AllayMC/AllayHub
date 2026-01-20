use super::builder::{build_plugins_from_repo, parse_github_url};
use crate::github::client;
use crate::plugin::Plugin;
use std::collections::HashSet;
use tracing::{debug, debug_span, info, warn};

pub struct UpdateResult {
    pub updated: Vec<Plugin>,
    pub unchanged: Vec<String>,
    pub deleted: Vec<String>,
    pub errors: Vec<(String, String)>,
    pub processed_ids: HashSet<String>,
    pub stopped_by_rate_limit: bool,
}

pub fn update_existing_plugins(plugins: &[Plugin], force: bool) -> UpdateResult {
    if plugins.is_empty() {
        return UpdateResult {
            updated: Vec::new(),
            unchanged: Vec::new(),
            deleted: Vec::new(),
            errors: Vec::new(),
            processed_ids: HashSet::new(),
            stopped_by_rate_limit: false,
        };
    }

    let batch = client().execute_parallel(plugins.to_vec(), move |plugin, _| {
        let _span = debug_span!("update_plugin", id = %plugin.id).entered();
        (plugin.id.clone(), update_plugin(&plugin, force))
    });

    let mut updated = Vec::new();
    let mut unchanged = Vec::new();
    let mut deleted = Vec::new();
    let mut errors = Vec::new();
    let mut processed_ids = HashSet::new();

    for (id, status) in batch.results {
        processed_ids.insert(id.clone());
        match status {
            Ok(UpdateStatus::Updated(plugin)) => updated.push(plugin),
            Ok(UpdateStatus::Unchanged) => unchanged.push(id),
            Ok(UpdateStatus::Deleted) => deleted.push(id),
            Err(e) => errors.push((id, e)),
        }
    }

    info!(
        processed = batch.processed,
        total = batch.total,
        api_remaining = client().rate_limit.remaining(),
        "Batch processed"
    );

    if batch.stopped_by_rate_limit {
        warn!(
            processed = batch.processed,
            total = batch.total,
            "Stopped due to rate limit"
        );
    }

    UpdateResult {
        updated,
        unchanged,
        deleted,
        errors,
        processed_ids,
        stopped_by_rate_limit: batch.stopped_by_rate_limit,
    }
}

#[derive(Debug)]
enum UpdateStatus {
    Updated(Plugin),
    Unchanged,
    Deleted,
}

fn update_plugin(plugin: &Plugin, force: bool) -> Result<UpdateStatus, String> {
    let (owner, repo_name) = match parse_github_url(&plugin.source) {
        Some(parts) => parts,
        None => return Ok(UpdateStatus::Unchanged),
    };

    let repo = match client().get_repository(&owner, &repo_name) {
        Ok(r) => r,
        Err(e) if e.contains("404") => {
            debug!(id = %plugin.id, "Plugin repo not found, marking deleted");
            return Ok(UpdateStatus::Deleted);
        }
        Err(e) => return Err(e),
    };

    if repo.archived {
        debug!(id = %plugin.id, "Plugin repo archived, marking deleted");
        return Ok(UpdateStatus::Deleted);
    }

    let new_plugins = build_plugins_from_repo(&repo, &[]);

    let new_plugin = new_plugins.into_iter().find(|p| p.id == plugin.id);

    let mut new_plugin = match new_plugin {
        Some(p) => p,
        None => {
            debug!(id = %plugin.id, "Plugin no longer in repo, marking deleted");
            return Ok(UpdateStatus::Deleted);
        }
    };

    merge_preserved_fields(plugin, &mut new_plugin);

    if force || plugin_changed(plugin, &new_plugin) {
        Ok(UpdateStatus::Updated(new_plugin))
    } else {
        Ok(UpdateStatus::Unchanged)
    }
}

fn merge_preserved_fields(old: &Plugin, new: &mut Plugin) {
    if old.preserved_fields.is_empty() {
        return;
    }

    let mut new_json = match serde_json::to_value(&*new) {
        Ok(serde_json::Value::Object(map)) => map,
        _ => return,
    };

    for (key, value) in &old.preserved_fields {
        new_json.insert(key.clone(), value.clone());
    }

    if let Ok(merged) = serde_json::from_value(serde_json::Value::Object(new_json)) {
        *new = merged;
    }

    new.preserved_fields = old.preserved_fields.clone();
}

fn plugin_changed(old: &Plugin, new: &Plugin) -> bool {
    old.name != new.name
        || old.summary != new.summary
        || old.stars != new.stars
        || old.downloads != new.downloads
        || old.license != new.license
        || old.authors != new.authors
        || versions_changed(&old.versions, &new.versions)
}

fn versions_changed(old: &[crate::plugin::Version], new: &[crate::plugin::Version]) -> bool {
    if old.len() != new.len() {
        return true;
    }

    for (o, n) in old.iter().zip(new.iter()) {
        if o.version != n.version || o.downloads != n.downloads {
            return true;
        }
    }

    false
}
