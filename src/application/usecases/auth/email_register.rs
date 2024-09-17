use std::sync::Arc;

use validator::Validate;

use crate::{
    application::dto::auth::email_request::EmailRegisterRequest,
    domain::{
        entities::{user::User, user_oauth_provider::UserOauthProvider, user_role::UserRole},
        repositories::{role_repo::RoleRepository, user_repo::UserRepository},
    },
    infra::{
        errors::app_error::AppError, oauth2::constants::EMAIL_PROVIDER,
        utils::password::hash_password,
    },
};

#[derive(Clone)]
pub struct EmailRegister<U, R> {
    user_repo: Arc<U>,
    role_repo: Arc<R>,
}

impl<U, R> EmailRegister<U, R>
where
    U: UserRepository,
    R: RoleRepository,
{
    pub fn new(user_repo: Arc<U>, role_repo: Arc<R>) -> Self {
        Self {
            user_repo,
            role_repo,
        }
    }

    pub async fn execute(
        &self,
        db_pool: &sqlx::PgPool,
        req: EmailRegisterRequest,
    ) -> Result<User, AppError> {
        req.validate()?;

        if self.user_repo.find_by_email(&req.email).await.is_ok() {
            return Err(AppError::UserEmailAlreadyExist);
        }

        let mut tx = db_pool.begin().await?;

        let default_role = self
            .role_repo
            .find_default()
            .await
            .map_err(|err| match err {
                AppError::SqlxError(sqlx::Error::RowNotFound) => AppError::ProcessError(
                    "Default Role is not Found please contact Administrator!".to_string(),
                ),
                _ => AppError::ProcessError(err.to_string()),
            })?;

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
        let user_role = UserRole::new(new_user.id.clone(), default_role.id.clone());

        let (user, _user_oauth_provider, _user_role) = self
            .user_repo
            .tx_register_user(&mut tx, &new_user, &user_oauth_provider, &user_role)
            .await
            .map_err(|err| AppError::ProcessError(err.to_string()))?;

        tx.commit().await?;

        Ok(user)
    }
}
