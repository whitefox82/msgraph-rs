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
msgraph-rs = "0.1.0"
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