use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use crate::{
    config::AppState,
    utils::{jwt::decode_token, scopes::enforce_scope},
    errors::AppError,
};

#[derive(Debug, Deserialize)]
pub struct AccessRequest {
    pub token: String,
    pub action: String,
}

/// Handler for POST /check-access
///
/// - Decodes JWT using secret from AppState
/// - Enforces scope based on the provided `action`
/// - Returns structured JSON result
pub async fn check_access_handler(
    State(state): State<AppState>,
    Json(payload): Json<AccessRequest>,
) -> impl IntoResponse {
    // ?? Step 1: Decode the token
    let decoded = match decode_token(&payload.token, &state.jwt_secret) {
        Ok(claims) => claims,
        Err(err) => return AppError::unauthorized(err).into_response(),
    };

    // ? Step 2: Enforce the requested action scope
    if let Err(reason) = enforce_scope(&decoded.scopes, &payload.action) {
        return AppError::forbidden(reason).into_response();
    }

    // ?? Step 3: Success
    let response = serde_json::json!({
        "allowed": true,
        "reason": "Access granted"
    });

    (StatusCode::OK, Json(response)).into_response()
}
