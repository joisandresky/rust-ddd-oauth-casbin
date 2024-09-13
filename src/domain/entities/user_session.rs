use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct UserSession {
    pub id: String,
    pub user_id: String,
    pub refresh_token: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl UserSession {
    pub fn new(
        user_id: String,
        refresh_token: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            refresh_token,
            expires_at,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn update(
        &mut self,
        refresh_token: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) {
        self.refresh_token = refresh_token;
        self.expires_at = expires_at;
    }
}
