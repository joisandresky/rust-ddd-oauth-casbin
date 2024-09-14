use std::{collections::HashMap, sync::Arc};

use axum::{extract::State, Json, Router};
use tokio::fs;

use crate::{
    application::state::AppState,
    infra::{errors::app_error::AppError, utils::response::SuccessResponse},
};

pub fn setup_permission_handler() -> Router<Arc<AppState>> {
    Router::new().route("/list", axum::routing::get(get_permission_list))
}

// TODO: change into caching or changes detection when performance issue happens
async fn get_permission_list(
    State(app_state): State<Arc<AppState>>,
) -> Result<SuccessResponse<HashMap<String, Vec<String>>>, AppError> {
    if let Ok(permissions) = app_state.svc.redis.get_permissions_list().await {
        return Ok(SuccessResponse::with_data(200, permissions));
    }

    let permission_list = fs::read_to_string("etc/permissions.json").await?;

    let permission_list: HashMap<String, Vec<String>> = serde_json::from_str(&permission_list)?;

    app_state
        .svc
        .redis
        .set_permissions_list(permission_list.clone())
        .await?;

    Ok(SuccessResponse::with_data(200, permission_list))
}
