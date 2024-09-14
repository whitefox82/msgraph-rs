use crate::client::GraphClient;
use serde_json::Value;
use std::error::Error;

pub fn create_user(client: &GraphClient, body: Value) -> Result<Value, Box<dyn Error>> {
    client.post("/users", body)
}

pub fn list_users(client: &GraphClient) -> Result<Value, Box<dyn Error>> {
    let response = client.get("/users")?;
    crate::pagination::fetch_all_pages(client, response)
}

pub fn get_user(client: &GraphClient, user_id: &str) -> Result<Value, Box<dyn Error>> {
    let path = format!("/users/{}", user_id);
    client.get(&path)
}

pub fn update_user(
    client: &GraphClient,
    user_id: &str,
    body: Value,
) -> Result<Value, Box<dyn Error>> {
    let path = format!("/users/{}", user_id);
    client.patch(&path, body)
}

pub fn revoke_sign_in_sessions(client: &GraphClient, user_id: &str) -> Result<Value, Box<dyn Error>> {
    let path = format!("/users/{}/revokeSignInSessions", user_id);
    client.post(&path, serde_json::json!({}))
}

pub fn send_mail(
    client: &GraphClient,
    user_id: &str,
    body: Value,
) -> Result<(), Box<dyn Error>> {
    let path = format!("/users/{}/sendMail", user_id);

    // Use `post_raw` to get the raw HTTP response
    let response = client.post_raw(&path, body)?;

    // Check if the response status is successful (2xx)
    if response.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to send email. Status: {}", response.status()),
        )))
    }
}

