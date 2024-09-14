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

    // Specify the user you want to retrieve.
    // Replace `<user_principal_name>` with the actual email (UPN) or the user's ID.
    // The userPrincipalName is usually the email address, but you can also use the object ID.
    let id = "<user_principal_name>";

    // Fetch the user's details using the `get_user` method.
    // This method sends a GET request to the `/users/{id}` endpoint, where `{id}` can be
    // either the user's principal name (email) or the user's ID in the Azure AD tenant.
    //
    // The response is a JSON object that contains various details about the user,
    // such as their display name, email, job title, etc.
    let user = get_user(&client, id)?;

    // Print the user details to the console.
    // This will display the user's information as returned by the Microsoft Graph API.
    println!("User: {}", user);

    Ok(())
}
