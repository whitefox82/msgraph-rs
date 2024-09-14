use base64::engine::general_purpose;
use base64::Engine;
use msgraph_rs::client::GraphClient;
use msgraph_rs::resources::users::methods::*;
use std::fs;

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

    // Load the attachment (xlsx file) and convert it to base64
    let file_path = "path/to/your/file.xlsx"; // Replace with your actual file path
    let file_content = fs::read(file_path)?;

    // Use the `Engine::encode` method with `general_purpose` engine to avoid the deprecation warning
    let base64_file_content = general_purpose::STANDARD.encode(file_content);

    // Define the email body with the attachment
    let mail_body = serde_json::json!({
        "message": {
            "subject": "<email_subject>", // Subject of the email
            "body": {
                "contentType": "Text",
                "content": "<email_content>" // The actual content of the email message
            },
            "toRecipients": [
                {
                    "emailAddress": {
                        "content": "<email_content>" // The actual content of the email message
                    }
                }
            ],
            "attachments": [
                {
                    "@odata.type": "#microsoft.graph.fileAttachment",
                    "name": "meeting-details.xlsx",
                    "contentType": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                    "contentBytes": base64_file_content
                }
            ]
        },
        "saveToSentItems": "true"
    });

    // Send the email with the attachment
    send_mail(&client, user_id, mail_body)?;

    println!("Email with attachment sent successfully");

    Ok(())
}
