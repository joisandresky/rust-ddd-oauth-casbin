use axum::{http::StatusCode, response::IntoResponse, Json};
use bb8_redis::{bb8::RunError, redis::RedisError};
use serde_json::json;
use thiserror::Error;
use tokio::task::JoinError;
use validator::ValidationErrors;

// Define a more structured error response body
#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
    pub success: bool,
}

// Define an AppError enum that scales
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("[Validation error] {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Redis Error: {0}")]
    RedisError(#[from] RedisError),

    #[error("Redis Connection Error: {0}")]
    RunRedisError(#[from] RunError<RedisError>),

    #[error("Set-Header Error: {0}")]
    FailedToSetHeader(#[from] axum::http::header::InvalidHeaderValue),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unauthorized Error: {0}")]
    UnauthorizedError(String),

    #[error("Json Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("JWT Error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    #[error("Process error: {0}")]
    ProcessError(String),

    #[error("Casbin Error: {0}")]
    CasbinError(#[from] casbin::Error),

    #[error("IO Error: {0}")]
    TokioFsError(#[from] tokio::io::Error),

    #[error("{0}")]
    NotFound(String),

    #[error("Resource Not Found")]
    ResourceNotFound,

    #[error("Resource Exist: {0}")]
    ResourceExist(String),

    #[error("Invalid Oauth Provider")]
    InvalidOauthProvider,

    #[error("HTTP Client Error: {0}")]
    HttpClientError(#[from] reqwest::Error),

    #[error("Oauth2 Failed to Authorize")]
    Oauth2FailedToAuthorize,

    #[error("Invalid Auth Token")]
    InvalidToken,

    #[error("User Email Already Exist")]
    UserEmailAlreadyExist,

    #[error("User with given email {0} not exist")]
    UserNotExist(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_code, message) = match &self {
            AppError::ProcessError(value) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "process_error".to_string(),
                format!("failed to process request: {}", value),
            ),
            AppError::ValidationError(_) => (
                StatusCode::BAD_REQUEST,
                "validation_error".to_string(),
                "failed to validate your request, please check & try again".to_string(),
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized".to_string(),
                "Unauthorized".to_string(),
            ),
            AppError::JWTError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "jwt_error".to_string(),
                "failed to validate/make token".to_string(),
            ),
            AppError::SqlxError(err) => match err {
                sqlx::Error::RowNotFound => (
                    StatusCode::NOT_FOUND,
                    "resource_not_found".to_string(),
                    "Resource not found!".to_string(),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error".to_string(),
                    "A database error occurred.".to_string(),
                ),
            },
            AppError::ResourceNotFound => (
                StatusCode::NOT_FOUND,
                "resource_not_found".to_string(),
                "The requested resource was not found.".to_string(),
            ),
            AppError::ResourceExist(value) => (
                StatusCode::BAD_REQUEST,
                "resource_exist".to_string(),
                format!("Resource already exist: {}", value),
            ),
            AppError::CasbinError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "casbin_error".to_string(),
                format!("Casbin error: {}", err),
            ),
            AppError::InvalidOauthProvider => (
                StatusCode::BAD_REQUEST,
                "invalid_oauth_provider".to_string(),
                "Invalid oauth provider".to_string(),
            ),
            AppError::Oauth2FailedToAuthorize => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "oauth2_failed_to_authorize".to_string(),
                "Oauth2 failed to authorize".to_string(),
            ),
            AppError::UnauthorizedError(value) => (
                StatusCode::UNAUTHORIZED,
                "unauthorized".to_string(),
                format!("Unauthorized: {}", value),
            ),
            AppError::UserEmailAlreadyExist => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "user_email_already_exist".to_string(),
                "User with given email already exist".to_string(),
            ),
            AppError::UserNotExist(value) => (
                StatusCode::BAD_REQUEST,
                "user_not_exist".to_string(),
                format!("User with given email {value} not exist"),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error".to_string(),
                "An internal server error occurred.".to_string(),
            ),
        };

        let body = Json(json!({
            "error_code": error_code,
            "message": message,
            "success": false,
        }));

        (status, body).into_response()
    }
}

// Convert specific errors into AppError variants
impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::ProcessError(value.to_string())
    }
}

impl From<JoinError> for AppError {
    fn from(value: JoinError) -> Self {
        AppError::ProcessError(value.to_string())
    }
}
