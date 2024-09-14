use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

const TOKEN_URL: &str = "https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token";

#[derive(Serialize)]
struct TokenRequest {
    client_id: String,
    scope: String,
    client_secret: String,
    grant_type: String,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

pub fn get_access_token(
    client_id: &str,
    client_secret: &str,
    tenant_id: &str,
) -> Result<String, Box<dyn Error>> {
    let url = TOKEN_URL.replace("{tenant_id}", tenant_id);

    let client = Client::new();
    let body = TokenRequest {
        client_id: client_id.to_string(),
        scope: "https://graph.microsoft.com/.default".to_string(),
        client_secret: client_secret.to_string(),
        grant_type: "client_credentials".to_string(),
    };

    let res: TokenResponse = client.post(&url).form(&body).send()?.json()?;

    Ok(res.access_token)
}
