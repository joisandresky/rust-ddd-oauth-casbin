use serde::Serialize;

use super::{role::Role, user_oauth_provider::UserOauthProvider};

#[derive(Clone, Debug, Serialize)]
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
    pub fn new(id: String, email: String, password_hash: Option<String>) -> Self {
        Self {
            id,
            email,
            password_hash,
            fullname: None,
            avatar_url: None,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
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

#[derive(Clone, Debug, Serialize)]
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
