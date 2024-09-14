use crate::resources::auth::token::get_access_token;
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;

pub struct GraphClient {
    client: Client,
    access_token: String,
    base_url: String,
}

impl GraphClient {
    // Initialize the GraphClient with the access token obtained using client credentials
    pub fn new(
        client_id: &str,
        client_secret: &str,
        tenant_id: &str,
        beta: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let base_url = if beta {
            crate::constants::GRAPH_URL_BETA
        } else {
            crate::constants::GRAPH_URL
        }
        .to_string();

        // Get the access token
        let access_token = get_access_token(client_id, client_secret, tenant_id)?;

        Ok(GraphClient {
            client: Client::new(),
            access_token,
            base_url,
        })
    }

    // Getter method for client
    pub fn get_http_client(&self) -> &Client {
        &self.client
    }

    // Getter method for access_token
    pub fn get_access_token(&self) -> &str {
        &self.access_token
    }

    // Function to send GET request
    pub fn get(&self, path: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, path);
        let res = self
            .client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()?
            .json()?;
        Ok(res)
    }

    // Function to send POST request
    pub fn post(&self, path: &str, body: Value) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, path);
        let res = self
            .client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&body)
            .send()?
            .json()?;
        Ok(res)
    }

    // Function to send DELETE request
    pub fn delete(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, path);
        self.client
            .delete(&url)
            .bearer_auth(&self.access_token)
            .send()?;
        Ok(())
    }

    // Function to send PATCH request
    pub fn patch(&self, path: &str, body: Value) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, path);
        let res = self
            .client
            .patch(&url)
            .bearer_auth(&self.access_token)
            .json(&body)
            .send()?
            .json()?;
        Ok(res)
    }
}
