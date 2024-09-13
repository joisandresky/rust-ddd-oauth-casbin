use std::sync::Arc;

use casbin::MgmtApi;

use crate::{
    application::dto::role::create_update_role_request::CreateOrUpdateRole,
    domain::repositories::role_repo::RoleRepository,
    infra::{errors::app_error::AppError, rbac::Rbac},
};

#[derive(Clone)]
pub struct UpdateRoleById<R> {
    role_repo: Arc<R>,
    rbac: Arc<Rbac>,
}

impl<R> UpdateRoleById<R>
where
    R: RoleRepository,
{
    pub fn new(role_repo: Arc<R>, rbac: Arc<Rbac>) -> Self {
        Self { role_repo, rbac }
    }

    pub async fn execute(&self, id: &str, req: CreateOrUpdateRole) -> Result<(), AppError> {
        if req.is_default && self.role_repo.find_default().await.is_ok() {
            return Err(AppError::ResourceExist(
                "Default role already exist".to_owned(),
            ));
        }

        let mut role = self.role_repo.find_by_id(id).await?;

        role.update(&req.name, req.is_default);

        let mut enforcer = self.rbac.enforcer.write().await;
        let current_policies = enforcer.get_filtered_policy(0, vec![role.id.clone()]);

        if let Some(permissions) = req.permissions {
            // Transform current policies into "permission_name:action" format
            let current_permissions: Vec<String> = current_policies
                .into_iter()
                .filter(|policy| policy.len() == 3)
                .map(|policy| format!("{}:{}", policy[1], policy[2]))
                .collect();

            // Determine permissions to add (in new but not in current)
            let permissions_to_add: Vec<&String> = permissions
                .iter()
                .filter(|perm| !current_permissions.contains(perm))
                .collect();

            // Determine permissions to remove (in current but not in new)
            let permissions_to_remove: Vec<&String> = current_permissions
                .iter()
                .filter(|perm| !permissions.contains(perm))
                .collect();

            // Add missing permissions
            for permission in permissions_to_add {
                let parts: Vec<&str> = permission.split(':').collect();
                if parts.len() == 2 {
                    let policy = vec![role.id.clone(), parts[0].to_string(), parts[1].to_string()];
                    println!("Adding policy: {:?}", policy);
                    enforcer.add_policy(policy).await?;
                }
            }

            // Remove extra permissions
            for permission in permissions_to_remove {
                let parts: Vec<&str> = permission.split(':').collect();
                if parts.len() == 2 {
                    let policy = vec![role.id.clone(), parts[0].to_string(), parts[1].to_string()];
                    println!("Removing policy: {:?}", policy);
                    enforcer.remove_policy(policy).await?;
                }
            }
        } else {
            // remove all policies that this role has
            for policy in current_policies {
                enforcer.remove_policy(policy).await?;
            }
        }

        Ok(())
    }
}
