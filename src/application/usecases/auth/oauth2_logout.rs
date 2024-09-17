use std::sync::Arc;

use crate::{
    application::services::oauth_svc::OauthService,
    domain::repositories::{
        oauth_provider_repo::OauthProviderRepository, role_repo::RoleRepository,
        user_repo::UserRepository, user_session_repo::UserSessionRepository,
    },
    infra::{
        errors::app_error::AppError,
        oauth2::constants::{EMAIL_PROVIDER, GOOGLE_PROVIDER},
    },
};

#[derive(Clone)]
pub struct Oauth2Logout<U, R, S, O> {
    user_session_repo: Arc<S>,
    oauth_svc: Arc<OauthService<U, R, S, O>>,
}

impl<U, R, S, O> Oauth2Logout<U, R, S, O>
where
    U: UserRepository,
    R: RoleRepository,
    S: UserSessionRepository,
    O: OauthProviderRepository,
{
    pub fn new(user_session_repo: Arc<S>, oauth_svc: Arc<OauthService<U, R, S, O>>) -> Self {
        Self {
            user_session_repo,
            oauth_svc,
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

        Ok(())
    }
}
