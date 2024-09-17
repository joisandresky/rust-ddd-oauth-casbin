use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{role::Role, user_oauth_provider::UserOauthProvider};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub fullname: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl User {
    pub fn new(email: String, password_hash: Option<String>) -> Self {
        let extracted_name_from_email = email
            .split('@')
            .collect::<Vec<&str>>()
            .first()
            .unwrap_or(&"")
            .to_string();

        Self {
            id: Uuid::new_v4().to_string(),
            email,
            password_hash,
            fullname: Some(extracted_name_from_email),
            avatar_url: None,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }

    pub fn change_email(&mut self, email: String) {
        self.email = email;
        self.updated_at = chrono::Utc::now();
    }

    pub fn update(&mut self, fullname: Option<String>, avatar_url: Option<String>) {
        self.fullname = fullname;
        self.avatar_url = avatar_url;
        self.updated_at = chrono::Utc::now();
    }

    pub fn delete(&mut self) {
        self.is_active = false;
        self.deleted_at = Some(chrono::Utc::now());
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserFull {
    #[serde(flatten)]
    pub user: User,

    pub oauth_provider: UserOauthProvider,
    pub roles: Vec<Role>,
}

impl UserFull {
    pub fn new(user: User, oauth_provider: UserOauthProvider, roles: Vec<Role>) -> Self {
        Self {
            user,
            oauth_provider,
            roles,
        }
    }
}
