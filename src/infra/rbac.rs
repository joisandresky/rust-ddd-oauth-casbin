use std::sync::Arc;

use casbin::{CoreApi, Enforcer, MgmtApi, RbacApi};
use tokio::sync::RwLock;
use tracing::info;

use crate::domain::entities::role::Role;

#[derive(Clone)]
pub struct Rbac {
    pub enforcer: Arc<RwLock<Enforcer>>,
}

impl Rbac {
    pub fn new(enforcer: Arc<RwLock<Enforcer>>) -> Self {
        Self { enforcer }
    }

    pub async fn check_access(
        &self,
        roles: &Vec<Role>,
        object: &str,
        action: &str,
    ) -> Result<bool, casbin::Error> {
        let roles = roles.clone();
        if roles.is_empty() {
            return Ok(false);
        }

        let enforcer = self.enforcer.read().await;

        // for now get the first one
        let subject = match roles.get(0) {
            Some(role) => role.id.clone(),
            None => String::default(),
        };

        let has_access = enforcer.enforce((subject, object, action))?;

        Ok(has_access)
    }

    pub async fn setup_roles_and_permissions(&self) {
        info!("Setting up Roles and Permissions...");

        let mut enforcer = self.enforcer.write().await;

        // Expected policies
        let expected_policies = vec![
            vec!["user".to_owned(), "public".to_owned(), "read".to_owned()],
            // Commented out, meaning it should be removed if it exists
            vec!["user".to_owned(), "public".to_owned(), "write".to_owned()],
            vec![
                "root".to_owned(),
                "user-management".to_owned(),
                "read".to_owned(),
            ],
            vec![
                "root".to_owned(),
                "user-management".to_owned(),
                "write".to_owned(),
            ],
        ];

        // Expected role hierarchies
        let expected_roles = vec![("root", "user")];

        // Get current policies from the enforcer
        let current_policies = enforcer.get_policy();

        // Detect and remove extra policies, but keep those with 'shared:<id>' pattern
        for policy in &current_policies {
            let is_shared_policy = policy[1].starts_with("shared:");
            if !expected_policies.contains(policy) && !is_shared_policy {
                info!("Removing extra policy {:?}", policy);
                enforcer.remove_policy(policy.clone()).await.unwrap();
            }
        }

        // Seed policies
        for policy in &expected_policies {
            if !enforcer.has_policy(policy.clone()) {
                enforcer.add_policy(policy.clone()).await.unwrap();
            }
        }

        // Get current roles from the enforcer
        for (parent_role, child_role) in &expected_roles {
            let current_roles = enforcer.get_roles_for_user(child_role, None);
            if !current_roles.contains(&parent_role.to_string()) {
                info!(
                    "Role hierarchy for {} -> {} has been deleted!",
                    child_role, parent_role
                );
                // Re-add the role if it was deleted
                enforcer
                    .add_role_for_user(child_role, parent_role, None)
                    .await
                    .unwrap();
            }
        }

        // Seed Role Hierarchies
        for (child_role, parent_role) in &expected_roles {
            if !enforcer.has_role_for_user(child_role, parent_role, None) {
                enforcer
                    .add_role_for_user(child_role, parent_role, None)
                    .await
                    .unwrap();
            }
        }

        info!("Roles and Permissions Setup Completed!");
    }
}
