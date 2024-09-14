use msgraph_rs::client::GraphClient;
use msgraph_rs::resources::users::methods::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // These are your Microsoft Azure app credentials.
    // Replace the values with your actual client ID, client secret, and tenant ID.
    let client_id = "";
    let client_secret = "";
    let tenant_id = "";

    // Initialize the `GraphClient` which allows you to interact with the Microsoft Graph API.
    let client = GraphClient::new(client_id, client_secret, tenant_id, false)?;

    // The user ID (or UPN - User Principal Name) of the sender.
    let user_id = "<sender_user_principal_name>"; // e.g., "user@example.com"

    // Define the email body. This is where you structure the content of the email to be sent.
    let mail_body = serde_json::json!({
        "message": {
            "subject": "<email_subject>", // Subject of the email
            "body": {
                "contentType": "HTML",        // Changed from "Text" to "HTML"
                "content": "<html><body><h1>Email Content</h1><p>This is an example email with <b>HTML</b> formatting.</p></body></html>" // The actual content of the email message in HTML
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
