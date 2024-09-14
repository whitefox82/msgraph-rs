# msgraph-rs

`msgraph-rs` is a Rust library for interacting with the Microsoft Graph API. It provides an easy-to-use interface for authenticating and making requests to Microsoft Graph endpoints, allowing you to manage users, groups, emails, and more within Azure Active Directory.

## Features

- **User management**: Create, list, retrieve, and manage users in your Azure AD tenant.
- **Authentication**: Authenticates using Microsoft Azure app credentials (client ID, client secret, and tenant ID).
- **Modular API support**: Supports different Microsoft Graph API features through a modular architecture.
- **Supports stable and beta API versions**: You can choose whether to use the stable or beta version of the Microsoft Graph API.

## Installation

To use this library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
msgraph-rs = "0.1.2"
```
or 

```bash
cargo add msgraph-rs
```

## Usage

### Authentication

To authenticate, you'll need to provide your Microsoft Azure app credentials (client ID, client secret, and tenant ID). These credentials allow you to interact with Microsoft Graph APIs securely.

You can find these values in the Azure portal by navigating to **Azure Active Directory** > **App registrations** and selecting your app.

### Example: Obtain Access Token

The following example demonstrates how to obtain an access token and output it to the terminal.

[access_token.rs](https://github.com/whitefox82/msgraph-rs/blob/main/examples/access_token.rs)

#### Required App Permissions:
- **Application**: `User.Read.All`
- **Delegated**: `User.Read`

These permissions allow the app to read user profiles in the directory and authenticate.

### Example: List Users

The following example demonstrates how to list users in your Azure Active Directory tenant using the `msgraph-rs` library.

[list_users.rs](https://github.com/whitefox82/msgraph-rs/blob/main/examples/list_users.rs)

#### Required App Permissions:
- **Application**: `User.Read.All`
- **Delegated**: `User.ReadBasic.All`

These permissions allow the app to read the basic profile information of all users in the directory. `User.Read.All` allows broader access, while `User.ReadBasic.All` allows access to minimal profile information (like display name and email).

### Example: Get a Specific User

This example shows how to retrieve a specific user by their `userPrincipalName` (typically their email) or `user_id`.

[get_user.rs](https://github.com/whitefox82/msgraph-rs/blob/main/examples/get_user.rs)

#### Required App Permissions:
- **Application**: `User.Read.All`
- **Delegated**: `User.Read`

These permissions allow the app to retrieve detailed profile information for a specific user in the directory.

### Example: Revoke User Sign in Sessions

This example shows how to revoke a specific users sign in sessions by their `userPrincipalName` (typically their email) or `user_id`.

[revoke_signin.rs](https://github.com/whitefox82/msgraph-rs/blob/main/examples/revoke_signin.rs)

#### Required App Permissions:
- **Application**: `Directory.ReadWrite.All`
- **Delegated**: `User.ReadWrite`

These permissions allow the app to read and revoke a user’s sign-in sessions in the directory.

### Example: Send Email

This example demonstrates how to send an email using the Microsoft Graph API. You will need to provide the sender's `userPrincipalName` (typically their email) and the email body including the recipient's email address, subject, and content.

- [send_email.rs](https://github.com/whitefox82/msgraph-rs/blob/main/examples/send_email.rs)
- [send_email_html.rs](https://github.com/whitefox82/msgraph-rs/blob/main/examples/send_email_html.rs)
- [send_email_attachment.rs](https://github.com/whitefox82/msgraph-rs/blob/main/examples/send_email_attachment.rs)

#### Required App Permissions:
- **Application**: `Mail.Send`
- **Delegated**: `Mail.Send`

To allow the app to send emails **on behalf of any user** in the organization, the `Mail.Send` application permission must be granted with **admin consent**. This permission allows the app to impersonate any user in the organization and send emails on their behalf.
