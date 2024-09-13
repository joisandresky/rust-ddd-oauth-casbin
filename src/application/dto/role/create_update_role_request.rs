use serde::Deserialize;
use validator::Validate;

use crate::domain::entities::role::Role;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateOrUpdateRole {
    #[validate(length(min = 1, max = 255, message = "Name is required"))]
    pub name: String,

    pub is_default: bool,

    pub permissions: Option<Vec<String>>,
}

impl From<&CreateOrUpdateRole> for Role {
    fn from(req: &CreateOrUpdateRole) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name.clone(),
            is_default: req.is_default,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
}
