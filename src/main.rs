use axum::Router;
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer, MakeRequestId, RequestId},
    trace::TraceLayer
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use uuid::Uuid;
use http::header::{HeaderName, HeaderValue};
use tower::ServiceBuilder;
use axum::http::Request;

// Internal modules
mod config;
mod routes;
mod handlers;
mod utils;
mod middleware;
mod errors;

#[derive(Clone, Default)]
struct RequestIdGenerator;

impl MakeRequestId for RequestIdGenerator {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let uuid = Uuid::new_v4().to_string();
        let header_value = HeaderValue::from_str(&uuid).ok()?;
        Some(RequestId::new(header_value))
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize logging and tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = config::AppState::from_env();

    // Build middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(RequestIdGenerator))
        .layer(PropagateRequestIdLayer::new(HeaderName::from_static("x-request-id")))
        .layer(TraceLayer::new_for_http());

    // Build app with routes and middleware
    let app = Router::new()
        .nest("/", routes::create_routes(app_state.clone()))
        .layer(middleware_stack);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("?? Agent Gateway running at http://{}/", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
