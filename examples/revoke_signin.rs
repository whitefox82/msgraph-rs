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

    // Replace this with the actual UPN (email address) or user ID of the target user.
    let upn = "";  // The User Principal Name (UPN) or user ID of the user whose sign-in sessions you want to revoke.

    // Call the `revoke_sign_in_sessions` function to revoke all sign-in sessions for the specified user.
    // The function sends a POST request to the Microsoft Graph API endpoint `/users/{user_id}/revokeSignInSessions`.
    // `client` is the initialized GraphClient, and `upn` is the UPN (email) or ID of the user.
    // The `?` operator is used to propagate any errors that may occur during the request.
    let response = revoke_sign_in_sessions(&client, upn)?;

    println!("Sign-in sessions revoked: {}", response);


    Ok(())
}
