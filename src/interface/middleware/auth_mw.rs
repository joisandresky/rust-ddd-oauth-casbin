use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    application::state::AppState,
    infra::{
        errors::app_error::AppError,
        oauth2::constants::{EMAIL_PROVIDER, GOOGLE_PROVIDER},
    },
};

pub async fn is_authorized(
    cookie_jar: CookieJar,
    State(app_state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("[Middleware:Auth->is_authorized] Checking if user is authorized");

    let token = cookie_jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string());

    if token.is_none() {
        return Err(AppError::Unauthorized);
    }

    let provider = cookie_jar
        .get("provider")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_default();

    if provider.is_empty() {
        return Err(AppError::InvalidOauthProvider);
    }

    // TODO: Implement Automatic Refresh Token when access_token expired
    // TODO: Implement Caching with Redis

    let (from_cache, current_user) = match provider.as_str() {
        GOOGLE_PROVIDER => {
            let claims = app_state
                .google_jwt_maker
                .verify_token(&token.unwrap_or_default())
                .await
                .map_err(|err| {
                    tracing::info!(
                        "[Middleware:Auth->is_authorized->GOOGLE_PROVIDER] User is not authorized with error: {}",
                        err
                    );
                    AppError::UnauthorizedError(err.to_string())
                })?;

            match app_state.svc.redis.get_current_user(&claims.sub).await {
                Ok(existing_current_user) => (true, existing_current_user),
                Err(_) => {
                    let current_user = app_state
                        .svc
                        .oauth
                        .get_current_oauth_user(&provider, &claims.sub)
                        .await
                        .map_err(|err| AppError::UnauthorizedError(err.to_string()))?;

                    (false, current_user)
                }
            }
        }
        EMAIL_PROVIDER => {
            let claims = app_state
                .jwt_maker
                .verify_access_token(&token.unwrap_or_default())
                .map_err(|err| {
                    tracing::info!(
                        "[Middleware:Auth->is_authorized->EMAIL_PROVIDER] User is not authorized with error: {}",
                        err
                    );
                    AppError::UnauthorizedError(err.to_string())
                })?;

            match app_state.svc.redis.get_current_user(&claims.sub).await {
                Ok(existing_current_user) => (true, existing_current_user),
                Err(_) => {
                    let current_user = app_state
                        .svc
                        .oauth
                        .get_current_oauth_user(&provider, &claims.sub)
                        .await
                        .map_err(|err| AppError::UnauthorizedError(err.to_string()))?;

                    (false, current_user)
                }
            }
        }
        _ => {
            tracing::info!("[Middleware:Auth->is_authorized] User is not authorized because of Invalid Oauth Provider");
            return Err(AppError::UnauthorizedError(
                "User is not authorized because of Invalid Oauth Provider".to_string(),
            ));
        }
    };

    if !from_cache {
        tracing::info!("dapet cache, return bro");
        app_state.svc.redis.set_current_user(&current_user).await?;
    }

    tracing::info!(
        "[Middleware:Auth->is_authorized] User is authorized {}",
        &current_user.user.id
    );

    req.extensions_mut().insert(current_user);

    let response = next.run(req).await;

    Ok(response.into_response())
}
