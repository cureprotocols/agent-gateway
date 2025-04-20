use std::env;
use std::sync::Arc;

/// Shared application state passed into routes and handlers
#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: Arc<String>,
}

impl AppState {
    /// Loads configuration from environment variables (.env) into AppState
    pub fn from_env() -> Self {
        let jwt_secret = env::var("JWT_SECRET")
            .expect("? JWT_SECRET must be set in .env");

        AppState {
            jwt_secret: Arc::new(jwt_secret),
        }
    }
}

