use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Role {
    pub fn new(id: String, name: String, is_default: bool) -> Self {
        Self {
            id,
            name,
            is_default,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }

    pub fn update(&mut self, name: &str, is_default: bool) {
        self.name = name.to_owned();
        self.is_default = is_default;
        self.updated_at = chrono::Utc::now();
    }
}
