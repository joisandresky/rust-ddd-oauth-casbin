use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    middleware,
    routing::get,
    Extension, Json, Router,
};

use crate::{
    application::{
        dto::role::{
            create_update_role_request::CreateOrUpdateRole, get_role_request::RoleWithPermission,
        },
        state::AppState,
    },
    domain::entities::{role::Role, user::UserFull},
    infra::{
        errors::app_error::AppError,
        utils::{
            pagination::{PaginatedResponse, PaginationQuery},
            response::SuccessResponse,
        },
    },
    interface::middleware::auth_mw::is_authorized,
};

pub fn setup_role_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/all", get(get_all_roles))
        .route("/", get(get_paginated_roles).post(create_role))
        .route(
            "/:id",
            get(get_role_by_id).put(update_role).delete(delete_role),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            is_authorized,
        ))
}

async fn get_paginated_roles(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<PaginationQuery>,
) -> Result<SuccessResponse<PaginatedResponse<Role>>, AppError> {
    let has_access = state
        .rbac
        .check_access(&current_user.roles, "role-management", "read")
        .await?;

    if !has_access {
        return Err(AppError::Forbidden);
    }

    let roles = state
        .uc
        .role
        .get_paginated_role
        .execute(
            query.page.unwrap_or(1 as i64),
            query.limit.unwrap_or(15 as i64),
        )
        .await?;

    Ok(SuccessResponse::with_data(200, roles))
}

async fn get_all_roles(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
) -> Result<SuccessResponse<Vec<Role>>, AppError> {
    let has_access = state
        .rbac
        .check_access(&current_user.roles, "role-management", "read")
        .await?;

    if !has_access {
        return Err(AppError::Forbidden);
    }

    let roles = state.uc.role.get_all_role.execute().await?;

    Ok(SuccessResponse::with_data(200, roles))
}

async fn get_role_by_id(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<SuccessResponse<RoleWithPermission>, AppError> {
    let has_access = state
        .rbac
        .check_access(&current_user.roles, "role-management", "read")
        .await?;

    if !has_access {
        return Err(AppError::Forbidden);
    }

    let role = state.uc.role.get_role_by_id.execute(&id).await?;

    Ok(SuccessResponse::with_data(200, role))
}

async fn create_role(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOrUpdateRole>,
) -> Result<SuccessResponse<String>, AppError> {
    let has_access = state
        .rbac
        .check_access(&current_user.roles, "role-management", "write")
        .await?;

    if !has_access {
        return Err(AppError::Forbidden);
    }

    let role = state.uc.role.create_role.execute(req).await?;

    Ok(SuccessResponse::with_data(200, role.id))
}

async fn update_role(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<CreateOrUpdateRole>,
) -> Result<SuccessResponse<String>, AppError> {
    let has_access = state
        .rbac
        .check_access(&current_user.roles, "role-management", "write")
        .await?;

    if !has_access {
        return Err(AppError::Forbidden);
    }

    state.uc.role.update_role_by_id.execute(&id, req).await?;

    Ok(SuccessResponse::with_data(200, id))
}

async fn delete_role(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<SuccessResponse<String>, AppError> {
    let has_access = state
        .rbac
        .check_access(&current_user.roles, "role-management", "write")
        .await?;

    if !has_access {
        return Err(AppError::Forbidden);
    }

    state.uc.role.delete_role_by_id.execute(&id).await?;

    Ok(SuccessResponse::with_data(200, id))
}
