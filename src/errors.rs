use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Unified error type for API-level responses
#[derive(Debug)]
pub enum AppError {
    Unauthorized(String),
    Forbidden(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl AppError {
    pub fn unauthorized(msg: String) -> Self {
        AppError::Unauthorized(msg)
    }

    pub fn forbidden(msg: String) -> Self {
        AppError::Forbidden(msg)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
        };

        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}

