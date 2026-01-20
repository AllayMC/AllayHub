use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Claims {
    iat: i64,
    exp: i64,
    iss: String,
}

#[derive(Deserialize)]
struct InstallationToken {
    token: String,
}

pub fn create_jwt(app_id: &str, private_key_pem: &str) -> Result<String, String> {
    let now = chrono::Utc::now().timestamp();

    let claims = Claims {
        iat: now - 60,
        exp: now + 600,
        iss: app_id.to_string(),
    };

    let key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
        .map_err(|e| format!("Invalid private key: {}", e))?;

    let header = Header::new(Algorithm::RS256);

    encode(&header, &claims, &key).map_err(|e| format!("JWT encode error: {}", e))
}

pub fn get_installation_token(jwt: &str, installation_id: &str) -> Result<String, String> {
    let url = format!(
        "https://api.github.com/app/installations/{}/access_tokens",
        installation_id
    );

    let mut resp = ureq::post(&url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", &format!("Bearer {}", jwt))
        .header("User-Agent", "allayindexer")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send_empty()
        .map_err(|e| format!("Request failed: {}", e))?;

    let token: InstallationToken = resp
        .body_mut()
        .read_json()
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(token.token)
}

#[derive(Clone)]
pub struct GitHubAppAuth {
    pub app_id: String,
    pub installation_id: String,
    pub private_key: String,
}

impl GitHubAppAuth {
    pub fn get_token(&self) -> Result<String, String> {
        let jwt = create_jwt(&self.app_id, &self.private_key)?;
        get_installation_token(&jwt, &self.installation_id)
    }
}
