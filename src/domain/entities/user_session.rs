use serde::Serialize;
use uuid::Uuid;

// TODO: Add to access_token
#[derive(Clone, Debug, Serialize)]
pub struct UserSession {
    pub id: String,
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl UserSession {
    pub fn new(
        user_id: String,
        access_token: String,
        refresh_token: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            access_token,
            refresh_token,
            expires_at,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn update(
        &mut self,
        access_token: String,
        refresh_token: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) {
        self.access_token = access_token;
        self.refresh_token = refresh_token;
        self.expires_at = expires_at;
    }
}
