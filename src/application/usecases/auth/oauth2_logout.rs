use std::sync::Arc;

use crate::{
    application::services::{oauth_svc::OauthService, redis_svc::RedisService},
    domain::repositories::{
        oauth_provider_repo::OauthProviderRepository, role_repo::RoleRepository,
        user_repo::UserRepository, user_session_repo::UserSessionRepository,
    },
    infra::{
        errors::app_error::AppError,
        oauth2::constants::{EMAIL_PROVIDER, GOOGLE_PROVIDER},
        repositories::redis_repo_impl::RedisRepositoryImpl,
    },
};

#[derive(Clone)]
pub struct Oauth2Logout<U, R, S, O> {
    user_session_repo: Arc<S>,
    oauth_svc: Arc<OauthService<U, R, S, O>>,
    redis_svc: Arc<RedisService<RedisRepositoryImpl>>,
}

impl<U, R, S, O> Oauth2Logout<U, R, S, O>
where
    U: UserRepository,
    R: RoleRepository,
    S: UserSessionRepository,
    O: OauthProviderRepository,
{
    pub fn new(
        user_session_repo: Arc<S>,
        oauth_svc: Arc<OauthService<U, R, S, O>>,
        redis_svc: Arc<RedisService<RedisRepositoryImpl>>,
    ) -> Self {
        Self {
            user_session_repo,
            oauth_svc,
            redis_svc,
        }
    }

    pub async fn execute(&self, provider: &str, user_id: &str) -> Result<(), AppError> {
        let user_session = self
            .user_session_repo
            .find_by_user_id(user_id)
            .await
            .map_err(|err| match err {
                AppError::SqlxError(sqlx::Error::RowNotFound) => AppError::Unauthorized,
                _ => err,
            })?;

        match provider {
            GOOGLE_PROVIDER => {
                self.oauth_svc
                    .google_revoke_token(&user_session.access_token)
                    .await?;
            }
            EMAIL_PROVIDER => {}
            _ => return Err(AppError::InvalidOauthProvider),
        }

        self.redis_svc.remove_current_user(user_id).await?;

        Ok(())
    }
}
