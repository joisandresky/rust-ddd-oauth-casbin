use std::{collections::HashMap, sync::Arc};

use chrono::TimeZone;
use tracing::info;

use crate::{
    application::dto::auth::oauth2_response::{
        GoogleTokenError, GoogleTokenResponse, GoogleUserResult,
    },
    domain::{
        entities::{
            user::{User, UserFull},
            user_oauth_provider::UserOauthProvider,
            user_role::UserRole,
            user_session::UserSession,
        },
        repositories::{
            oauth_provider_repo::OauthProviderRepository, role_repo::RoleRepository,
            user_repo::UserRepository, user_session_repo::UserSessionRepository,
        },
    },
    infra::{
        config::AppConfig,
        errors::app_error::AppError,
        oauth2::{constants::GOOGLE_PROVIDER, google::GOOGLE_TOKEN_ENDPOINT},
    },
};

#[derive(Clone)]
pub struct OauthService<U, R, S, O> {
    cfg: Arc<AppConfig>,
    user_repo: Arc<U>,
    role_repo: Arc<R>,
    user_session_repo: Arc<S>,
    oauth_provider_repo: Arc<O>,
}

impl<U, R, S, O> OauthService<U, R, S, O>
where
    U: UserRepository,
    R: RoleRepository,
    S: UserSessionRepository,
    O: OauthProviderRepository,
{
    pub fn new(
        cfg: Arc<AppConfig>,
        user_repo: Arc<U>,
        role_repo: Arc<R>,
        user_session_repo: Arc<S>,
        oauth_provider_repo: Arc<O>,
    ) -> Self {
        Self {
            cfg,
            user_repo,
            role_repo,
            user_session_repo,
            oauth_provider_repo,
        }
    }

    pub async fn google_login(
        &self,
        db_pool: &sqlx::PgPool,
        code: &str,
    ) -> Result<GoogleTokenResponse, AppError> {
        let mut data = HashMap::new();

        data.insert("code".to_string(), code.to_string());
        data.insert("client_id".to_string(), self.cfg.google_client_id.clone());
        data.insert(
            "client_secret".to_string(),
            self.cfg.google_client_secret.clone(),
        );
        data.insert(
            "redirect_uri".to_string(),
            self.cfg.google_redirect_url.clone(),
        );
        data.insert("grant_type".to_string(), "authorization_code".to_string());

        let client = reqwest::Client::new();

        let response = client.post(GOOGLE_TOKEN_ENDPOINT).form(&data).send().await;

        match response {
            Ok(r) => {
                if r.status().is_success() {
                    let resp = r.json::<GoogleTokenResponse>().await?;

                    let user_info = self
                        .fetch_google_user(&resp.id_token, &resp.access_token)
                        .await?;

                    // if user already registered just return google auth response
                    if let Ok(u) = self.user_repo.find_by_email(&user_info.email).await {
                        let _session = self
                            .get_or_create_session(
                                &u.id,
                                &resp.access_token,
                                &resp.refresh_token,
                                Some(60 * 60 * 24 * 7),
                            )
                            .await?;
                        return Ok(resp);
                    }

                    // register user first & attached role
                    let user_data = self.register_user_from_google(db_pool, &user_info).await?;
                    let _session = self
                        .get_or_create_session(
                            &user_data.id,
                            &resp.access_token,
                            &resp.refresh_token,
                            Some(60 * 60 * 24 * 7),
                        )
                        .await?;

                    Ok(resp)
                } else {
                    let err_resp = r.json::<GoogleTokenError>().await?;

                    tracing::error!("{:?}", err_resp);
                    if err_resp.error == "invalid_grant" {
                        return Err(AppError::Unauthorized);
                    }

                    Err(AppError::ProcessError(err_resp.error))
                }
            }
            Err(err) => {
                tracing::error!("{}", err);
                Err(AppError::Oauth2FailedToAuthorize)
            }
        }
    }

    pub async fn fetch_google_user(
        &self,
        id_token: &str,
        access_token: &str,
    ) -> Result<GoogleUserResult, AppError> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!(
                "https://www.googleapis.com/oauth2/v3/userinfo?alt=json&access_token={}",
                access_token
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;

        match response {
            Ok(r) => {
                if r.status().is_success() {
                    let resp = r.json::<GoogleUserResult>().await?;

                    Ok(resp)
                } else {
                    let err_resp = r.json::<GoogleTokenError>().await?;

                    tracing::error!("{:?}", err_resp);
                    if err_resp.error == "invalid_grant" {
                        return Err(AppError::Unauthorized);
                    }

                    Err(AppError::ProcessError(err_resp.error))
                }
            }
            Err(err) => {
                tracing::error!("{}", err);
                Err(AppError::Oauth2FailedToAuthorize)
            }
        }
    }

    pub async fn register_user_from_google(
        &self,
        db_pool: &sqlx::PgPool,
        google_user_info: &GoogleUserResult,
    ) -> Result<User, AppError> {
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

        let user = User::from(google_user_info);
        let user_oauth_provider = UserOauthProvider::new(
            user.id.clone(),
            GOOGLE_PROVIDER.to_string(),
            google_user_info.sub.clone(),
        );
        let user_role = UserRole::new(user.id.clone(), default_role.id.clone());

        // insert user, user_oauth_provider, user_role
        let (user, _user_oauth_provider, _user_role) = self
            .user_repo
            .tx_register_user(&mut tx, &user, &user_oauth_provider, &user_role)
            .await?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn get_or_create_session(
        &self,
        user_id: &str,
        access_token: &str,
        refresh_token: &str,
        expires_at: Option<i64>,
    ) -> Result<UserSession, AppError> {
        // convert expires_at from i64 into DateTime
        let expires_at =
            expires_at.map(
                |expires_at| match chrono::Utc.timestamp_opt(expires_at, 0) {
                    chrono::LocalResult::Single(expires_at) => expires_at,
                    _ => chrono::Utc::now(),
                },
            );

        if let Ok(mut exist_session) = self.user_session_repo.find_by_user_id(user_id).await {
            if exist_session.refresh_token != refresh_token {
                // change refresh token
                exist_session.update(
                    access_token.to_string(),
                    refresh_token.to_string(),
                    expires_at,
                );

                info!(
                    "Exist Session and Not Matched Refresh Token, Updated Session: {:?}",
                    exist_session
                );

                self.user_session_repo
                    .update_token(&exist_session)
                    .await
                    .map_err(|err| AppError::ProcessError(err.to_string()))?;
            }

            return Ok(exist_session);
        }

        let session = UserSession::new(
            user_id.to_string(),
            access_token.to_string(),
            refresh_token.to_string(),
            expires_at,
        );
        let session = self
            .user_session_repo
            .create(session)
            .await
            .map_err(|err| AppError::ProcessError(err.to_string()))?;

        Ok(session)
    }

    pub async fn get_current_oauth_user(
        &self,
        provider: &str,
        provider_user_id: &str, // could be google id or discord id
    ) -> Result<UserFull, AppError> {
        let oauth_provider = self
            .oauth_provider_repo
            .get_by_provider_and_id(provider, provider_user_id)
            .await
            .map_err(|err| {
                tracing::error!(" an error occurred when get oauth provider {}", err);
                err
            })?;

        let user = self
            .user_repo
            .find_by_id(&oauth_provider.user_id)
            .await
            .map_err(|err| {
                tracing::error!(" an error occurred when get user {}", err);
                err
            })?;

        let roles = self
            .role_repo
            .get_roles_by_user_id(&user.id)
            .await
            .map_err(|err| {
                tracing::error!(" an error occurred when get roles {}", err);
                err
            })?;

        let user_full = UserFull::new(user, oauth_provider, roles);

        Ok(user_full)
    }

    pub async fn google_revoke_token(&self, access_token: &str) -> Result<(), AppError> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://oauth2.googleapis.com/revoke?token={}",
            access_token
        );

        let _ = client
            .post(url)
            .send()
            .await
            .map_err(|err| AppError::ProcessError(err.to_string()))?;

        Ok(())
    }

    pub async fn google_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<GoogleTokenResponse, AppError> {
        let client = reqwest::Client::new();
        let mut payload = HashMap::new();
        payload.insert("client_id", self.cfg.google_client_id.as_str());
        payload.insert("client_secret", self.cfg.google_client_secret.as_str());
        payload.insert("refresh_token", refresh_token);
        payload.insert("grant_type", "refresh_token");

        let r = client
            .post("https://oauth2.googleapis.com/token")
            .json(&payload)
            .send()
            .await;

        match r {
            Ok(r) => {
                if r.status().is_success() {
                    let resp = r.json::<GoogleTokenResponse>().await?;
                    Ok(resp)
                } else {
                    let err_resp = r.json::<GoogleTokenError>().await?;

                    tracing::error!("{:?}", err_resp);
                    if err_resp.error == "invalid_grant" {
                        return Err(AppError::Unauthorized);
                    }

                    Err(AppError::ProcessError(err_resp.error))
                }
            }
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(AppError::ProcessError(err.to_string()))
            }
        }
    }
}
