use serde::Serialize;

use crate::domain::entities::role::Role;

#[derive(Debug, Clone, Serialize)]
pub struct RoleWithPermission {
    #[serde(flatten)]
    pub role: Role,
    pub permissions: Vec<String>,
}
