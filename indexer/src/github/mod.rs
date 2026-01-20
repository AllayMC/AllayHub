pub mod api;
pub mod auth;
pub mod types;

pub use api::{BatchResult, GitHubClient};
pub use auth::GitHubAppAuth;
pub use types::*;

use crate::util::get_arg;
use std::env;
use std::fs;
use std::sync::OnceLock;

static CLIENT: OnceLock<GitHubClient> = OnceLock::new();

pub fn init_client(args: &[String]) -> Result<(), String> {
    let client = create_client(args)?;
    CLIENT
        .set(client)
        .map_err(|_| "Client already initialized".to_string())
}

pub fn client() -> &'static GitHubClient {
    CLIENT.get().expect("GitHub client not initialized")
}

fn create_client(args: &[String]) -> Result<GitHubClient, String> {
    let app_id = get_arg(args, "--app-id").or_else(|| env::var("GITHUB_APP_ID").ok());
    let installation_id =
        get_arg(args, "--installation-id").or_else(|| env::var("GITHUB_INSTALLATION_ID").ok());
    let private_key_file = get_arg(args, "--private-key-file");
    let private_key_env = env::var("GITHUB_PRIVATE_KEY").ok();

    if let (Some(app_id), Some(installation_id)) = (app_id, installation_id) {
        let private_key = if let Some(path) = private_key_file {
            fs::read_to_string(&path).map_err(|e| format!("Failed to read private key: {}", e))?
        } else if let Some(key) = private_key_env {
            key
        } else {
            return Err("GitHub App requires --private-key-file or GITHUB_PRIVATE_KEY".to_string());
        };

        println!("Using GitHub App authentication (15,000 req/hour)");
        return Ok(GitHubClient::with_app(GitHubAppAuth {
            app_id,
            installation_id,
            private_key,
        }));
    }

    let token = get_arg(args, "--token").or_else(|| env::var("GITHUB_TOKEN").ok());
    if let Some(t) = token {
        println!("Using personal token authentication (5,000 req/hour)");
        return Ok(GitHubClient::new(Some(t)));
    }

    Err("No authentication provided. Use --token or GitHub App options.".to_string())
}
