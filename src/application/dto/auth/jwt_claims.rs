use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GoogleJwtClaims {
    pub iss: String,                 // Issuer, usually the Google accounts URL
    pub azp: String,                 // Authorized party, usually the client ID of your app
    pub aud: String,                 // Audience, usually your app's client ID
    pub sub: String,                 // Subject, which is the unique Google user ID
    pub hd: Option<String>,          // Hosted domain, if applicable (optional field)
    pub email: String,               // User's email
    pub email_verified: bool,        // Whether the email has been verified
    pub at_hash: Option<String>,     // Access token hash (optional)
    pub name: String,                // Full name of the user
    pub picture: String,             // URL of the user's profile picture
    pub given_name: Option<String>,  // Given name of the user (optional)
    pub family_name: Option<String>, // Family name of the user (optional)
    pub iat: u64,                    // Issued at timestamp (seconds since the epoch)
    pub exp: u64,                    // Expiration timestamp (seconds since the epoch)
}
