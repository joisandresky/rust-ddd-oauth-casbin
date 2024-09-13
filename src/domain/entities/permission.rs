use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub action: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Permission {
    pub fn new(id: String, name: String, action: String) -> Self {
        Self {
            id,
            name,
            action,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
}
