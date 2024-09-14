use msgraph_rs::client::GraphClient;
use msgraph_rs::resources::users::methods::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // These are your Microsoft Azure app credentials.
    // Replace the values with your actual client ID, client secret, and tenant ID.
    let client_id = ""; // Client ID from Azure
    let client_secret = ""; // Client secret from Azure
    let tenant_id = ""; // Tenant ID from Azure

    // Initialize the `GraphClient` which allows you to interact with Microsoft Graph API.
    // The `GraphClient::new` method takes four parameters:
    // 1. `client_id` - Your Azure AD app client ID.
    // 2. `client_secret` - Your Azure AD app client secret.
    // 3. `tenant_id` - Your Azure AD tenant ID.
    // 4. `false` - This flag determines whether to use the beta version of the Microsoft Graph API.
    //    Set this to `true` if you want to use the beta API. Here, we use the stable v1.0 API.
    let client = GraphClient::new(client_id, client_secret, tenant_id, false)?;

    // Get the access token that was retrieved during the initialization of the `GraphClient`.
    // The access token is needed to authenticate requests to the Microsoft Graph API.
    let access_token = client.get_access_token();

    // Print the access token to the console.
    // This is useful for debugging and verifying that authentication was successful.
    println!("Access Token: {}", access_token);

    Ok(())
}
