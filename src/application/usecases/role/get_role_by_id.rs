use std::sync::Arc;

use casbin::MgmtApi;

use crate::{
    application::dto::role::get_role_request::RoleWithPermission,
    domain::repositories::role_repo::RoleRepository,
    infra::{errors::app_error::AppError, rbac::Rbac},
};

#[derive(Clone)]
pub struct GetRoleById<R> {
    role_repo: Arc<R>,
    rbac: Arc<Rbac>,
}

impl<R> GetRoleById<R>
where
    R: RoleRepository,
{
    pub fn new(role_repo: Arc<R>, rbac: Arc<Rbac>) -> Self {
        Self { role_repo, rbac }
    }

    pub async fn execute(&self, id: &str) -> Result<RoleWithPermission, AppError> {
        let role = self.role_repo.find_by_id(id).await?;

        let policies = self
            .rbac
            .enforcer
            .read()
            .await
            .get_filtered_policy(0, vec![role.id.clone()]);

        let permissions = policies
            .into_iter()
            .filter(|policy| policy.len() == 3)
            .map(|policy| format!("{}:{}", policy[1], policy[2]))
            .collect::<Vec<String>>();

        let role_with_permissions = RoleWithPermission { role, permissions };

        Ok(role_with_permissions)
    }
}
