use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

pub mod auth;
pub mod dashboard_api;
pub mod ingest;
pub mod realtime;

#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "Service is healthy", body = HealthzResponse)
    )
)]
pub async fn healthz() -> (StatusCode, Json<HealthzResponse>) {
    (StatusCode::OK, Json(HealthzResponse { status: "ok".to_string() }))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthzResponse {
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/version",
    responses(
        (status = 200, description = "Service version", body = VersionResponse)
    )
)]
pub async fn version() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        git_sha: option_env!("GIT_SHA").unwrap_or("unknown").to_string(),
        build_time: option_env!("BUILD_TIME").unwrap_or("unknown").to_string(),
    })
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionResponse {
    pub version: String,
    pub git_sha: String,
    pub build_time: String,
}