use std::sync::Arc;

use crate::{
    domain::{entities::role::Role, repositories::role_repo::RoleRepository},
    infra::errors::app_error::AppError,
};

#[derive(Clone)]
pub struct GetAllRole<R> {
    role_repo: Arc<R>,
}

impl<R> GetAllRole<R>
where
    R: RoleRepository,
{
    pub fn new(role_repo: Arc<R>) -> Self {
        Self { role_repo }
    }

    pub async fn execute(&self) -> Result<Vec<Role>, AppError> {
        let roles = self.role_repo.find_all().await?;

        Ok(roles)
    }
}
