use serde::Deserialize;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct EmailRegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct EmailLoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}
