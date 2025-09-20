use axum::{
    extract::{Json, State, Extension},
    routing::{post},
    Router,
    http::{StatusCode},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{auth::api_key::AgentAuth, config::AppState};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct IngestEvent {
    #[validate(length(min = 1, message = "Event ID cannot be empty"))]
    pub id: String,
    #[validate(length(min = 1, message = "Agent ID cannot be empty"))]
    pub agent_id: String,
    pub timestamp: String,
    #[validate(length(min = 1, message = "Event type cannot be empty"))]
    pub event_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IngestBatchRequest {
    #[validate(length(min = 1, message = "Events list cannot be empty"))]
    pub events: Vec<IngestEvent>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IngestBatchResponse {
    pub message: String,
    pub ingested_count: usize,
    pub failed_count: usize,
}

#[utoipa::path(
    post,
    path = "/v1/ingest",
    request_body = IngestBatchRequest,
    responses(
        (status = 200, description = "Batch ingest successful", body = IngestBatchResponse),
        (status = 400, description = "Invalid ingest request")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn batch(
    State(_app_state): State<AppState>, // Accept AppState
    Extension(agent_auth): Extension<AgentAuth>,
    Json(payload): Json<IngestBatchRequest>,
) -> Result<Json<IngestBatchResponse>, StatusCode> {
    if let Err(e) = payload.validate() {
        tracing::warn!("Invalid ingest batch request: {:?}", e);
        return Err(StatusCode::BAD_REQUEST);
    }

    tracing::info!(
        "Ingest batch from agent {} (org_id: {}) with {} events",
        agent_auth.key_id,
        agent_auth.org_id,
        payload.events.len()
    );

    let mut ingested_count = 0;
    let mut failed_count = 0;

    for event in payload.events {
        if event.agent_id != agent_auth.key_id {
            tracing::warn!(
                "Event agent_id mismatch: event.agent_id={} != api_key.agent_id={}",
                event.agent_id,
                agent_auth.key_id
            );
            failed_count += 1;
            continue;
        }
        if let Err(e) = event.validate() {
            tracing::warn!("Invalid event in batch: {:?}", e);
            failed_count += 1;
            continue;
        }

        // Simulate processing and storing the event
        tracing::debug!(
            "Ingesting event: id={}, agent_id={}, event_type={}",
            event.id,
            event.agent_id,
            event.event_type
        );
        ingested_count += 1;
    }

    Ok(Json(IngestBatchResponse {
        message: "Batch ingest processed".to_string(),
        ingested_count,
        failed_count,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ingest", post(batch))
}