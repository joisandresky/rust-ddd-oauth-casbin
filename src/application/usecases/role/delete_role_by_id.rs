use std::sync::Arc;

use casbin::MgmtApi;
use tracing::info;

use crate::{
    domain::repositories::role_repo::RoleRepository,
    infra::{common::constants::SUPER_ADMIN_ROLE, errors::app_error::AppError, rbac::Rbac},
};

#[derive(Clone)]
pub struct DeleteRoleById<R> {
    role_repo: Arc<R>,
    rbac: Arc<Rbac>,
}

impl<R> DeleteRoleById<R>
where
    R: RoleRepository,
{
    pub fn new(role_repo: Arc<R>, rbac: Arc<Rbac>) -> Self {
        Self { role_repo, rbac }
    }

    pub async fn execute(&self, id: &str) -> Result<(), AppError> {
        let role = self.role_repo.find_by_id(id).await?;

        if role.name == SUPER_ADMIN_ROLE {
            return Err(AppError::ProcessError(
                "Cannot delete super admin role".to_string(),
            ));
        }

        info!("Deleting Role with id {}...", id);
        self.role_repo.delete(id).await?;
        let mut enforcer = self.rbac.enforcer.write().await;
        let current_policies = enforcer.get_filtered_policy(0, vec![role.id.clone()]);

        info!("Removing policies for Role with id {}...", id);
        for policy in current_policies {
            enforcer.remove_policy(policy).await?;
        }

        Ok(())
    }
}
