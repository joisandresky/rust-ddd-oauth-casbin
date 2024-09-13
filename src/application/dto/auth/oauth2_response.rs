use uuid::Uuid;

use crate::domain::entities::user::User;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GoogleTokenResponse {
    pub id_token: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GoogleTokenError {
    pub error: String,
    pub error_description: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GoogleUserResult {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub picture: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email_verified: bool,
}

impl From<&GoogleUserResult> for User {
    fn from(google_user: &GoogleUserResult) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            fullname: Some(google_user.name.clone()),
            email: google_user.email.clone(),
            password_hash: None,
            avatar_url: google_user.picture.clone(),
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
}
