use msgraph_rs::client::GraphClient;
use msgraph_rs::resources::users::methods::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // These are your Microsoft Azure app credentials.
    // Replace the values with your actual client ID, client secret, and tenant ID.
    let client_id = ""; // Client ID from Azure AD app
    let client_secret = ""; // Client secret from Azure AD app
    let tenant_id = ""; // Tenant ID from your Azure AD instance

    // Initialize the `GraphClient` which allows you to interact with the Microsoft Graph API.
    // The `GraphClient::new` method takes four parameters:
    // 1. `client_id` - Your Azure AD app client ID.
    // 2. `client_secret` - Your Azure AD app client secret.
    // 3. `tenant_id` - Your Azure AD tenant ID.
    // 4. `false` - This flag determines whether to use the beta version of the Microsoft Graph API.
    //    Set this to `true` if you want to use the beta API. Here, we use the stable v1.0 API.
    let client = GraphClient::new(client_id, client_secret, tenant_id, false)?;

    // The user ID (or UPN - User Principal Name) of the sender.
    let user_id = "<sender_user_principal_name>"; // e.g., "user@example.com"

    // Define the email body. This is where you structure the content of the email to be sent.
    let mail_body = serde_json::json!({
        "message": {
            "subject": "<email_subject>", // Subject of the email
            "body": {
                "contentType": "Text",        // Type of the email content (Text or HTML)
                "content": "<email_content>" // The actual content of the email message
            },
            "toRecipients": [
                {
                    "emailAddress": {
                        "address": "<recipient_email>" // Recipient's email address
                    }
                }
            ]
        },
        "saveToSentItems": "true" // Whether to save the sent email in the sender's Sent Items folder
    });

    // Call the `send_mail` function to send the email using the Microsoft Graph API.
    // The function takes in the `GraphClient`, the sender's user ID, and the email body (JSON).
    // If the email is sent successfully, no error will be thrown, and the function will complete.
    send_mail(&client, user_id, mail_body)?;

    // Print a confirmation message to the console if the email is sent successfully.
    println!("Email sent successfully");

    // Return `Ok(())` indicating the program completed successfully.
    Ok(())
}
