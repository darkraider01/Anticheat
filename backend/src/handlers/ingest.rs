use axum::{routing::post, Router, extract::Extension};
use axum::http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use crate::auth::api_key::AgentAuth;
use axum::extract::Json;

#[derive(Debug, Deserialize, ToSchema)]
pub struct IngestEvent {
    pub event_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct IngestBatchRequest {
    pub events: Vec<IngestEvent>,
}

#[utoipa::path(
    post,
    path = "/ingest/batch",
    request_body = IngestBatchRequest,
    responses(
        (status = 202, description = "Batch accepted"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("apiKeyAuth" = [])
    )
)]
pub async fn batch(
    Extension(agent_auth): Extension<AgentAuth>,
    Json(payload): Json<IngestBatchRequest>,
) -> StatusCode {
    tracing::info!(
        "Ingesting batch for org_id: {}, key_id: {}, events: {}",
        agent_auth.org_id,
        agent_auth.key_id,
        payload.events.len()
    );
    // TODO: Process events
    StatusCode::ACCEPTED
}

pub fn routes() -> Router {
    Router::new().route("/batch", post(batch))
}