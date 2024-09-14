use serde_json::Value;
use std::error::Error;

pub fn fetch_all_pages(
    client: &crate::client::GraphClient,
    initial_response: Value,
) -> Result<Value, Box<dyn Error>> {
    let mut items = Vec::new();
    let mut response = initial_response;

    loop {
        if let Some(value) = response.get("value") {
            if let Some(array) = value.as_array() {
                items.extend(array.clone());
            }
        }

        if let Some(next_link) = response.get("@odata.nextLink") {
            if let Some(next_url) = next_link.as_str() {
                response = client
                    .get_http_client() // Use getter for the HTTP client
                    .get(next_url)
                    .bearer_auth(client.get_access_token()) // Use getter for the access token
                    .send()?
                    .json()?;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(serde_json::json!({ "value": items }))
}
