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

pub fn delete_user(client: &GraphClient, user_id: &str) -> Result<(), Box<dyn Error>> {
    let path = format!("/users/{}", user_id);
    client.delete(&path)
}

pub fn update_user(
    client: &GraphClient,
    user_id: &str,
    body: Value,
) -> Result<Value, Box<dyn Error>> {
    let path = format!("/users/{}", user_id);
    client.patch(&path, body)
}
