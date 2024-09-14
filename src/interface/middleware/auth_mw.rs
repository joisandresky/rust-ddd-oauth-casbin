use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{application::state::AppState, infra::errors::app_error::AppError};

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

    let provider = cookie_jar
        .get("provider")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_default();

    // TODO: Implement Automatic Refresh Token when access_token expired

    match provider.as_str() {
        "google" => {
            let claims = app_state
                .google_jwt_maker
                .verify_token(&token.unwrap_or_default())
                .await
                .map_err(|err| {
                    tracing::info!(
                        "[Middleware:Auth->is_authorized] User is not authorized with error: {}",
                        err
                    );
                    AppError::UnauthorizedError(err.to_string())
                })?;

            let current_user = app_state
                .svc
                .oauth
                .get_current_oauth_user(&provider, &claims.sub)
                .await
                .map_err(|err| AppError::UnauthorizedError(err.to_string()))?;

            req.extensions_mut().insert(current_user);
        }
        _ => {
            tracing::info!("[Middleware:Auth->is_authorized] User is not authorized because of Invalid Oauth Provider");
            return Err(AppError::UnauthorizedError(
                "User is not authorized because of Invalid Oauth Provider".to_string(),
            ));
        }
    }

    let response = next.run(req).await;

    Ok(response.into_response())
}
