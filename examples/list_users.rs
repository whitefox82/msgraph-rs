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

    // Call the `list_users` function from the `users` module.
    // This function makes a request to the `/users` endpoint of Microsoft Graph to retrieve
    // a list of users in the tenant. It uses the initialized `GraphClient` for authentication.
    let users = list_users(&client)?;

    // Print the list of users to the console.
    // The result is a JSON response from the Microsoft Graph API containing details
    // about the users in your tenant (e.g., display name, email).
    println!("Users: {}", users);

    Ok(())
}
