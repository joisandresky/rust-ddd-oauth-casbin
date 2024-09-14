use std::sync::Arc;

use axum::{
    extract::State,
    http::header,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{delete, get},
    Extension, Router,
};
use axum_extra::extract::cookie::{self, Cookie, Expiration};
use time::OffsetDateTime;

use crate::{
    application::state::AppState,
    domain::entities::user::UserFull,
    infra::{errors::app_error::AppError, utils::response::SuccessResponse},
    interface::middleware::auth_mw::is_authorized,
};

pub fn setup_auth_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/current-user", get(current_user))
        .route("/logout", delete(logout))
        .layer(from_fn_with_state(app_state, is_authorized))
}

pub async fn current_user(
    Extension(current_user): Extension<UserFull>,
) -> Result<SuccessResponse<UserFull>, AppError> {
    Ok(SuccessResponse::with_data(200, current_user))
}

pub async fn logout(
    Extension(current_user): Extension<UserFull>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    app_state
        .uc
        .auth
        .oauth2_logout
        .execute(&current_user.oauth_provider.provider, &current_user.user.id)
        .await?;

    let mut access_cookie = Cookie::build(("access_token", ""))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .expires(Expiration::from(OffsetDateTime::now_utc()));

    let mut refresh_cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .expires(Expiration::from(OffsetDateTime::now_utc()));

    let mut provider_cookie = Cookie::build(("provider", ""))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .expires(Expiration::from(OffsetDateTime::now_utc()));

    if &app_state.cfg.app_env != "local" {
        access_cookie = access_cookie.secure(true);
        refresh_cookie = refresh_cookie.secure(true);
        provider_cookie = provider_cookie.secure(true);
    }

    let mut resp = SuccessResponse::with_data(200, ()).into_response();

    tracing::info!("[API:Auth->logout] User logged out successfully");

    resp.headers_mut()
        .append(header::SET_COOKIE, access_cookie.to_string().parse()?);
    resp.headers_mut()
        .append(header::SET_COOKIE, refresh_cookie.to_string().parse()?);
    resp.headers_mut()
        .append(header::SET_COOKIE, provider_cookie.to_string().parse()?);

    Ok(resp)
}
