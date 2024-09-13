use std::sync::Arc;

use axum::{middleware::from_fn_with_state, routing::get, Extension, Router};

use crate::{
    application::state::AppState,
    domain::entities::user::UserFull,
    infra::{errors::app_error::AppError, utils::response::SuccessResponse},
    interface::middleware::auth_mw::is_authorized,
};

pub fn setup_auth_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/current-user", get(current_user))
        .layer(from_fn_with_state(app_state, is_authorized))
}

pub async fn current_user(
    Extension(current_user): Extension<UserFull>,
) -> Result<SuccessResponse<UserFull>, AppError> {
    Ok(SuccessResponse::with_data(200, current_user))
}
