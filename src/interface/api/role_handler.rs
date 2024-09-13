use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    application::{
        dto::role::{
            create_update_role_request::CreateOrUpdateRole, get_role_request::RoleWithPermission,
        },
        state::AppState,
    },
    domain::entities::role::Role,
    infra::{errors::app_error::AppError, utils::response::SuccessResponse},
};

pub fn setup_role_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_roles).post(create_role))
        .route(
            "/:id",
            get(get_role_by_id).put(update_role).delete(delete_role),
        )
}

async fn get_all_roles(
    State(state): State<Arc<AppState>>,
) -> Result<SuccessResponse<Vec<Role>>, AppError> {
    let roles = state.uc.role.get_all_role.execute().await?;

    Ok(SuccessResponse::with_data(200, roles))
}

async fn get_role_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<SuccessResponse<RoleWithPermission>, AppError> {
    let role = state.uc.role.get_role_by_id.execute(&id).await?;

    Ok(SuccessResponse::with_data(200, role))
}

async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOrUpdateRole>,
) -> Result<SuccessResponse<String>, AppError> {
    let role = state.uc.role.create_role.execute(req).await?;

    Ok(SuccessResponse::with_data(200, role.id))
}

async fn update_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<CreateOrUpdateRole>,
) -> Result<SuccessResponse<String>, AppError> {
    state.uc.role.update_role_by_id.execute(&id, req).await?;

    Ok(SuccessResponse::with_data(200, id))
}

async fn delete_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<SuccessResponse<String>, AppError> {
    state.uc.role.delete_role_by_id.execute(&id).await?;

    Ok(SuccessResponse::with_data(200, id))
}
