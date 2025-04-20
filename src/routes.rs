use axum::{routing::post, Router};
use crate::{
    handlers::check_access::check_access_handler,
    config::AppState,
};

/// Creates and returns the route definitions for the application.
///
/// # Routes
/// - POST /check-access ? Token validation and scope enforcement
pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/check-access", post(check_access_handler))
        .with_state(state)
}
