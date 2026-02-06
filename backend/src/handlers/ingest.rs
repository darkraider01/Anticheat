use axum::{
    extract::Extension,
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::{auth::api_key::AgentAuth, config::AppState};

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IngestBatchRequest {
    #[validate(length(min = 1, max = 1000, message = "Events array must contain 1-1000 items"))]
    pub events: Vec<DetectionEvent>,
    pub heartbeat: Option<AgentHeartbeat>,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Serialize)]
pub struct DetectionEvent {
    #[validate(length(min = 1, max = 100, message = "Event type must be 1-100 characters"))]
    pub event_type: String,
    
    #[validate(length(min = 1, max = 50, message = "Severity must be specified"))]
    pub severity: String,
    
    #[validate(length(max = 500, message = "Title too long"))]
    pub title: Option<String>,
    
    #[validate(length(max = 2000, message = "Description too long"))]
    pub description: Option<String>,
    
    pub metadata: serde_json::Value,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AgentHeartbeat {
    pub agent_version: String,
    pub platform: String,
    pub cpu_usage: Option<f32>,
    pub memory_usage: Option<f32>,
    pub last_scan_at: Option<DateTime<Utc>>,
    pub scan_count: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IngestResponse {
    pub success: bool,
    pub processed: u32,
    pub failed: u32,
    pub errors: Vec<String>,
}

/// Ingest batch of detection events from agents
#[utoipa::path(
    post,
    path = "/ingest/batch",
    request_body = IngestBatchRequest,
    responses(
        (status = 202, description = "Batch accepted for processing", body = IngestResponse),
        (status = 400, description = "Invalid request format"),
        (status = 401, description = "Invalid or missing API key"),
        (status = 413, description = "Payload too large")
    ),
    security(("apiKeyAuth" = [])),
    tag = "Ingest"
)]
pub async fn batch_ingest(
    Extension(agent_auth): Extension<AgentAuth>,
    Json(payload): Json<IngestBatchRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate the entire payload
    if let Err(validation_errors) = payload.validate() {
        tracing::warn!("Ingest validation failed: {:?}", validation_errors);
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(IngestResponse {
                success: false,
                processed: 0,
                failed: payload.events.len() as u32,
                errors: vec!["Invalid request format".to_string()],
            })
        ));
    }

    let org_id = &agent_auth.org_id;
    let agent_id = &agent_auth.agent_id;
    
    tracing::info!(
        "Processing {} events from agent {} in org {}",
        payload.events.len(),
        agent_id,
        org_id
    );

    let mut processed = 0u32;
    let mut failed = 0u32;
    let mut errors = Vec::new();

    // Process each event
    for event in &payload.events {
        // Validate individual event
        if let Err(e) = event.validate() {
            tracing::warn!("Event validation failed: {:?}", e);
            errors.push("Invalid event format".to_string());
            failed += 1;
            continue;
        }

        // TODO: In production:
        // 1. Store event in database with org_id and agent_id
        // 2. Enqueue event ID for async rule processing
        // 3. Publish to real-time channel for dashboard updates
        
        // For now, just log and increment counter
        tracing::info!(
            "Event processed: type={}, severity={}, org_id={}, agent_id={}",
            event.event_type,
            event.severity,
            org_id,
            agent_id
        );
        
        processed += 1;
    }

    // Process heartbeat if present
    if let Some(heartbeat) = &payload.heartbeat {
        if let Err(e) = heartbeat.validate() {
            tracing::warn!("Heartbeat validation failed: {:?}", e);
            errors.push("Invalid heartbeat format".to_string());
        } else {
            // TODO: Update agent status in database
            tracing::info!(
                "Agent heartbeat: version={}, platform={}, org_id={}, agent_id={}",
                heartbeat.agent_version,
                heartbeat.platform,
                org_id,
                agent_id
            );
        }
    }

    let success = failed == 0;
    let status_code = if success { StatusCode::ACCEPTED } else { StatusCode::PARTIAL_CONTENT };

    Ok((
        status_code,
        Json(IngestResponse {
            success,
            processed,
            failed,
            errors,
        })
    ))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/batch", axum::routing::post(batch_ingest))
}