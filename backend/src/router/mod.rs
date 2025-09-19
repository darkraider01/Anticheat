use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::{handlers, openapi};

pub fn create_router() -> Router {
    Router::new()
        .route("/healthz", get(handlers::healthz))
        .route("/version", get(handlers::version))
        .nest("/auth", handlers::auth::routes())
        .nest("/ingest", handlers::ingest::routes().layer(axum::middleware::from_fn(crate::auth::api_key::api_key_middleware))) // with API-key middleware
        .nest("/v1", handlers::dashboard_api::routes().layer(axum::middleware::from_fn(crate::auth::jwt::jwt_middleware))) // with JWT middleware
        .nest("/ws", handlers::realtime::routes())
        .route("/api-docs/openapi.json", get(openapi::serve_openapi)) // Serve OpenAPI JSON directly
        // Removed Swagger UI routes due to dependency conflicts
        .layer(TraceLayer::new_for_http())
}