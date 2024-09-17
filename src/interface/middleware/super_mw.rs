use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use base64::Engine;

use crate::{application::state::AppState, infra::errors::app_error::AppError};

pub async fn is_super_user(
    State(app_state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("[Middleware:Super->is_super_user] Checking if user is super user");

    let token = req
        .headers()
        .get("super_key")
        .map(|token| token.to_str().unwrap_or_default().to_string())
        .unwrap_or_default();

    if token.is_empty() {
        return Err(AppError::Unauthorized);
    }

    // decode super key
    let decoded_key = match Engine::decode(&base64::engine::general_purpose::STANDARD, &token) {
        Ok(decoded_bytes) => String::from_utf8(decoded_bytes).unwrap_or_default(),
        Err(err) => {
            tracing::info!(
                "[Middleware:Super->is_super_user] User is not authorized with error: {}",
                err
            );
            return Err(AppError::UnauthorizedError(err.to_string()));
        }
    };

    if decoded_key != app_state.cfg.super_key {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(req).await)
}
