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
        utils::jwt_maker::JwtMaker,
    },
};

pub struct RefreshOauthToken<U, R, S, O> {
    jwt_maker: Arc<JwtMaker>,
    user_session_repo: Arc<S>,
    oauth_svc: Arc<OauthService<U, R, S, O>>,
}

impl<U, R, S, O> RefreshOauthToken<U, R, S, O>
where
    U: UserRepository,
    R: RoleRepository,
    S: UserSessionRepository,
    O: OauthProviderRepository,
{
    pub fn new(
        jwt_maker: Arc<JwtMaker>,
        user_session_repo: Arc<S>,
        oauth_svc: Arc<OauthService<U, R, S, O>>,
    ) -> Self {
        Self {
            jwt_maker,
            user_session_repo,
            oauth_svc,
        }
    }

    pub async fn execute(
        &self,
        provider: &str,
        refresh_token: &str,
    ) -> Result<(String, String), AppError> {
        match provider {
            GOOGLE_PROVIDER => self.google_refresh_token(refresh_token).await,
            EMAIL_PROVIDER => self.email_refresh_token(refresh_token).await,
            _ => Err(AppError::InvalidOauthProvider),
        }
    }

    async fn google_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(String, String), AppError> {
        let mut session = self
            .user_session_repo
            .find_by_refresh_token(refresh_token)
            .await
            .map_err(|err| match err {
                AppError::SqlxError(sqlx::Error::RowNotFound) => {
                    AppError::UnauthorizedError("Invalid Session, try to relogin".to_string())
                }
                _ => AppError::ProcessError(err.to_string()),
            })?;

        if let Some(expire_at) = session.expires_at {
            if expire_at < chrono::Utc::now() {
                return Err(AppError::RefreshTokenExpired);
            }
        }

        let r = self.oauth_svc.google_refresh_token(refresh_token).await?;
        session.update(
            r.access_token.clone(),
            r.refresh_token.clone(),
            Some(chrono::Utc::now() + chrono::Duration::seconds(60 * 60 * 24 * 7)),
        );

        self.user_session_repo.update_token(&session).await?;

        Ok((r.id_token, r.refresh_token))
    }

    async fn email_refresh_token(&self, refresh_token: &str) -> Result<(String, String), AppError> {
        let claims = self
            .jwt_maker
            .verify_refresh_token(refresh_token)
            .map_err(|_| AppError::RefreshTokenExpired)?;

        let mut session = self
            .user_session_repo
            .find_by_user_id(&claims.sub)
            .await
            .map_err(|err| match err {
                AppError::SqlxError(sqlx::Error::RowNotFound) => {
                    AppError::UnauthorizedError("Invalid Session, try to relogin".to_string())
                }
                _ => AppError::ProcessError(err.to_string()),
            })?;

        if let Some(expire_at) = session.expires_at {
            if expire_at < chrono::Utc::now() {
                return Err(AppError::RefreshTokenExpired);
            }
        }

        let new_access_token = self.jwt_maker.make_token(claims.sub.clone(), 1)?;
        let new_refresh_token = self
            .jwt_maker
            .make_refresh_token(claims.sub.clone(), 24 * 7)?;

        let in_a_week = chrono::Utc::now() + chrono::Duration::seconds(60 * 60 * 24 * 7);
        session.update(
            new_access_token.clone(),
            new_refresh_token.clone(),
            Some(in_a_week),
        );

        self.user_session_repo.update_token(&session).await?;

        Ok((new_access_token, new_refresh_token))
    }
}
