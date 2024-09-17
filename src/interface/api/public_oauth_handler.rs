use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::header,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{self, Cookie};

use crate::{
    application::{
        dto::auth::{
            email_request::{EmailLoginRequest, EmailRegisterRequest},
            oauth2_request::Oauth2Request,
        },
        state::AppState,
    },
    infra::{
        errors::app_error::AppError,
        oauth2::constants::{EMAIL_PROVIDER, GOOGLE_PROVIDER},
        utils::response::SuccessResponse,
    },
};

pub fn setup_public_oauth_handler() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:provider/get-url", get(get_oauth_url))
        .route("/:provider/callback", get(handle_oauth2_callback))
        .route("/:provider/intercept", get(intercept_oauth_code))
        .route("/email/register", post(register_with_email))
        .route("/email/login", post(login_with_email))
}

pub async fn get_oauth_url(
    State(app_state): State<Arc<AppState>>,
    Path(provider): Path<String>,
) -> Result<SuccessResponse<String>, AppError> {
    if provider == GOOGLE_PROVIDER {
        let url = app_state.uc.auth.get_google_auth_url.execute().await?;

        return Ok(SuccessResponse::with_data(200, url));
    }

    Err(AppError::Unauthorized)
}

pub async fn handle_oauth2_callback(
    State(app_state): State<Arc<AppState>>,
    Path(provider): Path<String>,
    Query(req): Query<Oauth2Request>,
) -> Result<impl IntoResponse, AppError> {
    let (access_token, refresh_token) = app_state
        .uc
        .auth
        .oauth2_login
        .execute(&app_state.db_pool, provider.clone(), req)
        .await?;

    let mut access_cookie = Cookie::build(("access_token", access_token.clone()))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax);

    let mut refresh_cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax);

    let mut provider_cookie = Cookie::build(("provider", provider.clone()))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax);

    if &app_state.cfg.app_env != "local" {
        access_cookie = access_cookie.secure(true);
        refresh_cookie = refresh_cookie.secure(true);
        provider_cookie = provider_cookie.secure(true);
    }

    let mut resp = SuccessResponse::<u16>::with_code(200).into_response();

    tracing::info!("access_token {}", access_token);

    resp.headers_mut()
        .append(header::SET_COOKIE, access_cookie.to_string().parse()?);
    resp.headers_mut()
        .append(header::SET_COOKIE, refresh_cookie.to_string().parse()?);
    resp.headers_mut()
        .append(header::SET_COOKIE, provider_cookie.to_string().parse()?);

    Ok(resp)
}

/* this function only for testing on postman
*
*
* use this endpoint into your env GOOGLE_REDIRECT_URI so you can intercept request code
* and run in postman for handle_oauth2_callback
*
*
* */
pub async fn intercept_oauth_code(
    Query(req): Query<Oauth2Request>,
) -> Result<SuccessResponse<String>, AppError> {
    Ok(SuccessResponse::with_data(200, req.code))
}

/*
*
*
* EMAIL PROVIDER
*
* */

pub async fn register_with_email(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<EmailRegisterRequest>,
) -> Result<SuccessResponse<String>, AppError> {
    let user = app_state
        .uc
        .auth
        .email_register
        .execute(&app_state.db_pool, req)
        .await?;

    Ok(SuccessResponse::with_data(200, user.id))
}

pub async fn login_with_email(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<EmailLoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (access_token, refresh_token) = app_state.uc.auth.email_login.execute(req).await?;

    let mut access_cookie = Cookie::build(("access_token", access_token.clone()))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax);

    let mut refresh_cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax);

    let mut provider_cookie = Cookie::build(("provider", EMAIL_PROVIDER.to_string()))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax);

    if &app_state.cfg.app_env != "local" {
        access_cookie = access_cookie.secure(true);
        refresh_cookie = refresh_cookie.secure(true);
        provider_cookie = provider_cookie.secure(true);
    }

    let mut resp = SuccessResponse::<u16>::with_code(200).into_response();

    tracing::info!("access_token {}", access_token);

    resp.headers_mut()
        .append(header::SET_COOKIE, access_cookie.to_string().parse()?);
    resp.headers_mut()
        .append(header::SET_COOKIE, refresh_cookie.to_string().parse()?);
    resp.headers_mut()
        .append(header::SET_COOKIE, provider_cookie.to_string().parse()?);

    Ok(resp)
}
