use axum::{routing::get, Router, middleware};
use tower_http::trace::TraceLayer;
use crate::{handlers, openapi, config::AppState, auth::{jwt::jwt_middleware, api_key::api_key_middleware}};

pub fn create_router(app_state: AppState) -> Router {
    let api_routes = handlers::dashboard_api::routes()
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt_middleware,
        ));

    let ingest_routes = handlers::ingest::routes()
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            api_key_middleware,
        ));

    Router::new()
        .nest("/auth", handlers::auth::routes())
        .nest("/v1", api_routes)
        .nest("/ingest", ingest_routes)
        .nest("/realtime", handlers::realtime::routes())
        .route("/", get(|axum::extract::State(_app_state): axum::extract::State<AppState>| async { "CluelyGuard API v1.0" }))
        .route("/healthz", get(handlers::healthz))
        .route("/version", get(handlers::version))
        .route("/api-docs/openapi.json", get(openapi::serve_openapi))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}
