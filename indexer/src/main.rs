use allayindexer::codegen::write_allayhub_ts;
use allayindexer::github::{client, init_client};
use allayindexer::plugin::{load_plugins, write_plugin};
use allayindexer::search::build_orama_index;
use allayindexer::sync::{discover_new_plugins, update_existing_plugins};
use allayindexer::util::{
    clear_processed_ids, extract_repo_full_name, has_flag, read_last_sync, read_processed_ids,
    write_last_sync, write_processed_ids,
};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use tracing::{debug, error, info, info_span, warn};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let debug = has_flag(&args, "--debug");
    init_tracing(debug);

    match args[1].as_str() {
        "build" => cmd_build(),
        "update" => cmd_update(&args[2..]),
        "discover" => cmd_discover(&args[2..]),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            error!(command = %args[1], "Unknown command");
            print_usage();
            process::exit(1);
        }
    }
}

fn init_tracing(debug: bool) {
    use tracing_subscriber::{EnvFilter, fmt};

    let filter = if debug {
        EnvFilter::new("allayindexer=debug,info")
    } else {
        EnvFilter::new("info")
    };

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_span_events(if debug {
            fmt::format::FmtSpan::CLOSE
        } else {
            fmt::format::FmtSpan::NONE
        })
        .init();
}

fn print_usage() {
    println!("AllayHub Indexer v0.2.0");
    println!();
    println!("Usage:");
    println!("  allayindexer build                    Build search index only");
    println!("  allayindexer update [OPTIONS]         Update existing plugins");
    println!("  allayindexer discover [OPTIONS]       Discover new plugins");
    println!();
    println!("Options:");
    println!("  --force                      Force full run (ignore saved state)");
    println!("  --dry-run                    Preview changes without applying");
    println!("  --debug                      Enable debug logging");
    println!();
    println!("Authentication (choose one):");
    println!("  --token <TOKEN>              Personal access token (or GITHUB_TOKEN env)");
    println!();
    println!("  GitHub App (15,000 req/hour):");
    println!("  --app-id <ID>                App ID (or GITHUB_APP_ID env)");
    println!("  --installation-id <ID>       Installation ID (or GITHUB_INSTALLATION_ID env)");
    println!("  --private-key-file <PATH>    Path to .pem file (or GITHUB_PRIVATE_KEY env)");
}

fn cmd_build() {
    let _span = info_span!("build").entered();

    let index_dir = Path::new("AllayHubIndex");
    let output_file = Path::new("src/public/orama-index.bin");
    let builder_path = Path::new("orama_builder.mjs");
    let allayhub_ts_path = Path::new("src/types/allayhub.ts");

    if !index_dir.exists() {
        error!(path = ?index_dir, "Index directory not found");
        process::exit(1);
    }

    let plugins = {
        let _span = info_span!("load_plugins").entered();
        load_plugins(index_dir)
    };
    if plugins.is_empty() {
        warn!("No plugins found");
        return;
    }
    info!(count = plugins.len(), "Loaded plugins");

    {
        let _span = info_span!("generate_ts").entered();
        if !write_allayhub_ts(allayhub_ts_path) {
            process::exit(1);
        }
    }

    {
        let _span = info_span!("build_orama").entered();
        if !build_orama_index(&plugins, output_file, builder_path) {
            process::exit(1);
        }
    }

    if let Ok(meta) = fs::metadata(output_file) {
        let size_kb = meta.len() as f64 / 1024.0;
        info!(path = ?output_file, size_kb = format!("{:.1}", size_kb), "Index built");
    }
}

fn cmd_update(args: &[String]) {
    let _span = info_span!("update").entered();

    if let Err(e) = init_client(args) {
        error!(error = %e, "Failed to create client");
        process::exit(1);
    }

    let dry_run = has_flag(args, "--dry-run");
    let force = has_flag(args, "--force");
    let index_dir = Path::new("AllayHubIndex");

    if !index_dir.exists() {
        error!(path = ?index_dir, "Index directory not found");
        process::exit(1);
    }

    let plugins = {
        let _span = info_span!("load_plugins").entered();
        load_plugins(index_dir)
    };
    info!(count = plugins.len(), "Loaded plugins");

    let (remaining, mut processed_ids) = if force {
        info!("Force mode: updating all plugins");
        clear_processed_ids();
        (plugins.clone(), HashSet::new())
    } else {
        let processed = read_processed_ids();
        let remaining: Vec<_> = plugins
            .iter()
            .filter(|p| !processed.contains(&p.id))
            .cloned()
            .collect();
        (remaining, processed)
    };

    if remaining.is_empty() {
        info!("All plugins already updated today");
        clear_processed_ids();
        return;
    }
    info!(count = remaining.len(), "Plugins to update");

    let update = {
        let _span = info_span!("update_plugins", count = remaining.len()).entered();
        update_existing_plugins(&remaining, force)
    };

    if dry_run {
        if update.deleted.is_empty() {
            info!("No invalid plugins found");
        } else {
            info!(count = update.deleted.len(), "Would remove plugins");
            for id in &update.deleted {
                debug!(id = %id, "Would remove");
            }
        }

        if update.updated.is_empty() {
            info!("No updates available");
        } else {
            info!(count = update.updated.len(), "Would update plugins");
            for plugin in &update.updated {
                debug!(id = %plugin.id, "Would update");
            }
        }
    } else {
        for plugin in &update.updated {
            debug!(id = %plugin.id, "Updated");
            if let Err(e) = write_plugin(plugin, index_dir) {
                error!(id = %plugin.id, error = %e, "Failed to write plugin");
            }
        }

        for id in &update.deleted {
            debug!(id = %id, "Deleted");
            let path = index_dir.join(format!("{}.json", id));
            if let Err(e) = fs::remove_file(&path) {
                error!(id = %id, error = %e, "Failed to delete plugin");
            }
        }

        processed_ids.extend(update.processed_ids);

        if update.stopped_by_rate_limit {
            write_processed_ids(&processed_ids);
            warn!(processed = processed_ids.len(), "Stopped due to rate limit");
        } else {
            clear_processed_ids();
        }
    }

    for (id, err) in &update.errors {
        error!(id = %id, error = %err, "Plugin error");
    }

    info!(
        mode = if dry_run { "preview" } else { "complete" },
        removed = update.deleted.len(),
        updated = update.updated.len(),
        unchanged = update.unchanged.len(),
        api_calls = client().api_calls(),
        cache_hits = client().cache_hits(),
        api_remaining = client().rate_limit.remaining(),
        "Update finished"
    );

    client().export_data_cache().save();
}

fn cmd_discover(args: &[String]) {
    let _span = info_span!("discover").entered();

    if let Err(e) = init_client(args) {
        error!(error = %e, "Failed to create client");
        process::exit(1);
    }

    let dry_run = has_flag(args, "--dry-run");
    let index_dir = Path::new("AllayHubIndex");

    if !index_dir.exists() {
        error!(path = ?index_dir, "Index directory not found");
        process::exit(1);
    }

    let plugins = {
        let _span = info_span!("load_plugins").entered();
        load_plugins(index_dir)
    };
    info!(count = plugins.len(), "Loaded existing plugins");

    let existing_ids: HashSet<String> = plugins.iter().map(|p| p.id.clone()).collect();
    let existing_repos: HashSet<String> = plugins
        .iter()
        .filter_map(|p| extract_repo_full_name(&p.source))
        .collect();

    let last_sync = if has_flag(args, "--force") {
        info!("Full scan mode");
        None
    } else {
        read_last_sync()
    };
    if let Some(ref date) = last_sync {
        info!(since = %date, "Incremental scan");
    }

    let discover = {
        let _span = info_span!("discover_plugins").entered();
        discover_new_plugins(&existing_ids, &existing_repos, last_sync.as_deref())
    };

    if dry_run {
        if discover.new_plugins.is_empty() {
            info!("No new plugins found");
        } else {
            info!(count = discover.new_plugins.len(), "Would add plugins");
            for plugin in &discover.new_plugins {
                debug!(name = %plugin.name, id = %plugin.id, "Would add");
            }
        }
    } else {
        for plugin in &discover.new_plugins {
            debug!(name = %plugin.name, id = %plugin.id, "New plugin");
            if let Err(e) = write_plugin(plugin, index_dir) {
                error!(id = %plugin.id, error = %e, "Failed to write plugin");
            }
        }
        write_last_sync();
    }

    for (name, err) in &discover.errors {
        error!(repo = %name, error = %err, "Discover error");
    }

    info!(
        mode = if dry_run { "preview" } else { "complete" },
        found = discover.new_plugins.len(),
        api_calls = client().api_calls(),
        cache_hits = client().cache_hits(),
        api_remaining = client().rate_limit.remaining(),
        "Discover finished"
    );

    client().export_data_cache().save();
}
