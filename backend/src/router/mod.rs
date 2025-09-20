use axum::{
    middleware,
    routing::{get},
    Router,
};

use crate::{
    auth::{jwt::{jwt_middleware}, api_key::{api_key_middleware}},
    handlers::{auth, dashboard_api, ingest, realtime},
    config::AppState,
};
use axum::extract::State; // Keep State import for root route handler

pub fn create_router(app_state: AppState) -> Router {
    let api_routes = dashboard_api::routes()
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt_middleware,
        ))
        .with_state(app_state.clone());

    let ingest_routes = ingest::routes()
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            api_key_middleware,
        ))
        .with_state(app_state.clone());

    Router::new()
        .nest("/auth", auth::routes().with_state(app_state.clone()))
        .nest("/v1", api_routes)
        .nest("/ingest", ingest_routes)
        .nest("/realtime", realtime::routes().with_state(app_state.clone()))
        .route("/", get(|State(_app_state): State<AppState>| async { "Hello, World!" }))
        .with_state(app_state)
}