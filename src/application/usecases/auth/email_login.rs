use std::sync::Arc;

use validator::Validate;

use crate::{
    application::{dto::auth::email_request::EmailLoginRequest, services::oauth_svc::OauthService},
    domain::repositories::{
        oauth_provider_repo::OauthProviderRepository, role_repo::RoleRepository,
        user_repo::UserRepository, user_session_repo::UserSessionRepository,
    },
    infra::{
        errors::app_error::AppError,
        utils::{jwt_maker::JwtMaker, password::verify_password},
    },
};

#[derive(Clone)]
pub struct EmailLogin<U, R, S, O> {
    user_repo: Arc<U>,
    jwt_maker: Arc<JwtMaker>,
    oauth_svc: Arc<OauthService<U, R, S, O>>,
}

impl<U, R, S, O> EmailLogin<U, R, S, O>
where
    U: UserRepository,
    R: RoleRepository,
    S: UserSessionRepository,
    O: OauthProviderRepository,
{
    pub fn new(
        user_repo: Arc<U>,
        jwt_maker: Arc<JwtMaker>,
        oauth_svc: Arc<OauthService<U, R, S, O>>,
    ) -> Self {
        Self {
            user_repo,
            jwt_maker,
            oauth_svc,
        }
    }

    // TODO: implement single sign on ? so when new user login, other session will be terminated
    pub async fn execute(&self, req: EmailLoginRequest) -> Result<(String, String), AppError> {
        req.validate()?;

        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await
            .map_err(|err| match err {
                AppError::SqlxError(sqlx::Error::RowNotFound) => AppError::UserNotExist(req.email),
                _ => AppError::ProcessError(err.to_string()),
            })?;

        let cloned_pass = req.password.clone();
        tokio::task::spawn_blocking(move || {
            verify_password(
                &user.password_hash.unwrap_or_default(),
                cloned_pass.as_bytes(),
            )
        })
        .await?
        .map_err(|_err| AppError::UnauthorizedError(String::from("Invalid Credentials")))?;

        let access_token = self.jwt_maker.make_token(user.id.clone(), 1)?;
        let refresh_token = self.jwt_maker.make_refresh_token(user.id.clone(), 24 * 7)?;

        let _session = self
            .oauth_svc
            .get_or_create_session(
                &user.id,
                &access_token,
                &refresh_token,
                Some(60 * 60 * 24 * 7),
            )
            .await?;

        Ok((access_token, refresh_token))
    }
}
