use std::sync::Arc;

use crate::{
    application::{dto::auth::oauth2_request::Oauth2Request, services::oauth_svc::OauthService},
    domain::repositories::{
        oauth_provider_repo::OauthProviderRepository, role_repo::RoleRepository,
        user_repo::UserRepository, user_session_repo::UserSessionRepository,
    },
    infra::{errors::app_error::AppError, oauth2::constants::GOOGLE_PROVIDER},
};

#[derive(Clone)]
pub struct Oauth2Login<U, R, S, O> {
    oauth_svc: Arc<OauthService<U, R, S, O>>,
}

impl<U, R, S, O> Oauth2Login<U, R, S, O>
where
    U: UserRepository,
    R: RoleRepository,
    S: UserSessionRepository,
    O: OauthProviderRepository,
{
    pub fn new(oauth_svc: Arc<OauthService<U, R, S, O>>) -> Self {
        Self { oauth_svc }
    }

    // user can register/login
    pub async fn execute(
        &self,
        db_pool: &sqlx::PgPool,
        provider: String,
        req: Oauth2Request,
    ) -> Result<(String, String), AppError> {
        if provider == GOOGLE_PROVIDER {
            let google_resp = self.oauth_svc.google_login(db_pool, &req.code).await?;

            return Ok((google_resp.id_token, google_resp.refresh_token));
        }

        Err(AppError::InvalidOauthProvider)
    }
}
