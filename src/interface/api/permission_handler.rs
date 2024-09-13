use std::{collections::HashMap, sync::Arc};

use axum::{Json, Router};
use tokio::fs;

use crate::{application::state::AppState, infra::errors::app_error::AppError};

pub fn setup_permission_handler() -> Router<Arc<AppState>> {
    Router::new().route("/list", axum::routing::get(get_permission_list))
}

// TODO: change into caching or changes detection when performance issue happens
async fn get_permission_list() -> Result<Json<HashMap<String, Vec<String>>>, AppError> {
    let permission_list = fs::read_to_string("etc/permissions.json").await?;

    let permission_list: HashMap<String, Vec<String>> = serde_json::from_str(&permission_list)?;

    Ok(Json(permission_list))
}
