use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct UserRole {
    pub user_id: String,
    pub role_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl UserRole {
    pub fn new(user_id: String, role_id: String) -> Self {
        Self {
            user_id,
            role_id,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
