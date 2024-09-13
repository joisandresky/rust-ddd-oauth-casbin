use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> SuccessResponse<T>
where
    T: Serialize,
{
    pub fn new(code: u16, message: Option<&'static str>, data: Option<T>) -> Self {
        SuccessResponse {
            success: true,
            code,
            message,
            data,
        }
    }

    pub fn with_code(code: u16) -> Self {
        Self {
            success: true,
            code,
            message: None,
            data: None,
        }
    }

    pub fn with_message(code: u16, message: &'static str) -> Self {
        Self {
            success: true,
            code,
            message: Some(message),
            data: None,
        }
    }

    pub fn with_data(code: u16, data: T) -> Self {
        Self {
            success: true,
            code,
            message: None,
            data: Some(data),
        }
    }
}

impl<T> IntoResponse for SuccessResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let json_body = axum::Json(self);

        // Convert Json to Response
        let mut response = json_body.into_response();

        // Set the correct status code
        *response.status_mut() = code;

        response
    }
}
