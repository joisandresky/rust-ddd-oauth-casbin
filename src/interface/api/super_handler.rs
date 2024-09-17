use std::sync::Arc;

use axum::{extract::State, middleware, routing::post, Json, Router};

use crate::{
    application::{dto::auth::email_request::EmailRegisterRequest, state::AppState},
    infra::{errors::app_error::AppError, utils::response::SuccessResponse},
    interface::middleware::super_mw::is_super_user,
};

pub fn setup_super_handler(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/seed-super-user", post(seed_super_admin))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            is_super_user,
        ))
}

pub async fn seed_super_admin(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<EmailRegisterRequest>,
) -> Result<SuccessResponse<()>, AppError> {
    app_state
        .uc
        .auth
        .seed_super_admin
        .execute(&app_state.db_pool, req)
        .await?;

    Ok(SuccessResponse::with_data(200, ()))
}
