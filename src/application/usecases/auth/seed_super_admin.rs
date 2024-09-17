use std::{collections::HashMap, sync::Arc};

use casbin::MgmtApi;
use tokio::fs;
use uuid::Uuid;

use crate::{
    application::{
        dto::auth::email_request::EmailRegisterRequest, services::redis_svc::RedisService,
    },
    domain::{
        entities::{
            role::Role, user::User, user_oauth_provider::UserOauthProvider, user_role::UserRole,
        },
        repositories::{role_repo::RoleRepository, user_repo::UserRepository},
    },
    infra::{
        common::constants::SUPER_ADMIN_ROLE, errors::app_error::AppError,
        oauth2::constants::EMAIL_PROVIDER, rbac::Rbac,
        repositories::redis_repo_impl::RedisRepositoryImpl, utils::password::hash_password,
    },
};

#[derive(Clone)]
pub struct SeedSuperAdmin<U, R> {
    user_repo: Arc<U>,
    role_repo: Arc<R>,
    redis_svc: Arc<RedisService<RedisRepositoryImpl>>,
    rbac: Arc<Rbac>,
}

impl<U, R> SeedSuperAdmin<U, R>
where
    U: UserRepository,
    R: RoleRepository,
{
    pub fn new(
        user_repo: Arc<U>,
        role_repo: Arc<R>,
        redis_svc: Arc<RedisService<RedisRepositoryImpl>>,
        rbac: Arc<Rbac>,
    ) -> Self {
        Self {
            user_repo,
            role_repo,
            redis_svc,
            rbac,
        }
    }

    pub async fn execute(
        &self,
        db_pool: &sqlx::PgPool,
        req: EmailRegisterRequest,
    ) -> Result<(), AppError> {
        // Dissallow existing user to be Super Admin
        if self.user_repo.find_by_email(&req.email).await.is_ok() {
            return Err(AppError::ResourceExist(format!(
                "User with email {} already exist",
                req.email
            )));
        }

        let mut tx = db_pool.begin().await?;

        let super_role = match self.role_repo.find_by_name(SUPER_ADMIN_ROLE).await {
            Ok(role) => role,
            Err(err) => match err {
                AppError::SqlxError(sqlx::Error::RowNotFound) => {
                    let new_super_admin_role = Role::new(
                        Uuid::new_v4().to_string(),
                        SUPER_ADMIN_ROLE.to_string(),
                        false,
                    );

                    let mut enforcer = self.rbac.enforcer.write().await;

                    let policy = vec![
                        new_super_admin_role.id.clone(),
                        "*".to_string(),
                        "*".to_string(),
                    ];
                    enforcer.add_policy(policy).await?;

                    self.role_repo
                        .tx_create(&mut tx, new_super_admin_role)
                        .await?
                }
                _ => return Err(err),
            },
        };

        let cloned_pass = req.password.clone();
        let hashed_pass =
            tokio::task::spawn_blocking(move || hash_password(cloned_pass.as_bytes())).await??;

        let new_user = User::new(req.email, Some(hashed_pass));
        // for email provider we set provider_user_id same like user_id
        let user_oauth_provider = UserOauthProvider::new(
            new_user.id.clone(),
            EMAIL_PROVIDER.to_string(),
            new_user.id.clone(),
        );
        let user_role = UserRole::new(new_user.id.clone(), super_role.id.clone());

        let (_user, _user_oauth_provider, _user_role) = self
            .user_repo
            .tx_register_user(&mut tx, &new_user, &user_oauth_provider, &user_role)
            .await
            .map_err(|err| AppError::ProcessError(err.to_string()))?;

        tx.commit().await?;

        Ok(())
    }
}
