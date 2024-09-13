use std::sync::Arc;

use casbin::MgmtApi;

use crate::{
    application::dto::role::create_update_role_request::CreateOrUpdateRole,
    domain::{entities::role::Role, repositories::role_repo::RoleRepository},
    infra::{errors::app_error::AppError, rbac::Rbac},
};

#[derive(Clone)]
pub struct CreateRole<R> {
    role_repo: Arc<R>,
    rbac: Arc<Rbac>,
}

impl<R> CreateRole<R>
where
    R: RoleRepository,
{
    pub fn new(role_repo: Arc<R>, rbac: Arc<Rbac>) -> Self {
        Self { role_repo, rbac }
    }

    pub async fn execute(&self, req: CreateOrUpdateRole) -> Result<Role, AppError> {
        if req.is_default && self.role_repo.find_default().await.is_ok() {
            return Err(AppError::ResourceExist(
                "Default role already exist".to_owned(),
            ));
        }

        let role_req = Role::from(&req);

        let mut enforcer = self.rbac.enforcer.write().await;
        if let Some(permissions) = req.permissions {
            for permission in permissions {
                let permission_action = permission.split(':').collect::<Vec<&str>>();
                let policy = vec![
                    role_req.id.clone(),
                    permission_action[0].to_string(),
                    permission_action[1].to_string(),
                ];

                println!("Adding policy: {:?}", policy);

                let _ = enforcer.add_policy(policy).await;
            }
        }

        let role = self.role_repo.create(role_req).await?;

        Ok(role)
    }
}
