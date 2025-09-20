use axum::{
    routing::{post, get},
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade}, Extension
    },
    response::IntoResponse,
    Router,
};
use axum::http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use crate::auth::api_key::AgentAuth;
use axum::extract::Json;
use validator::{Validate, ValidationErrors, ValidationError, ValidationErrorsKind};
use futures::StreamExt;
use tracing::info;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct IngestEvent {
    #[validate(length(min = 1, message = "Event type cannot be empty"))]
    pub event_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
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
) -> Result<StatusCode, (StatusCode, String)> {
    // Validate the incoming batch request
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    // Filter events based on agent_auth.org_id
    let scoped_events: Vec<IngestEvent> = payload.events.into_iter().filter(|event| {
        // Assuming event.payload contains an "org_id" field for filtering
        // This is a placeholder and needs to be adapted to the actual payload structure
        if let Some(org_id_value) = event.payload.get("org_id") {
            if let Some(event_org_id) = org_id_value.as_str() {
                return event_org_id == agent_auth.org_id;
            }
        }
        // If no org_id in payload, or it doesn't match, filter it out
        false
    }).collect();

    if scoped_events.is_empty() {
        tracing::warn!("No events found for org_id: {}", agent_auth.org_id);
        return Ok(StatusCode::ACCEPTED);
    }

    for event in scoped_events {
        tracing::info!(
            "Ingesting event for org_id: {}, key_id: {}...{}, event_type: {}",
            agent_auth.org_id,
            &agent_auth.key_id[..4],
            &agent_auth.key_id[agent_auth.key_id.len()-4..],
            event.event_type
        );
        // TODO: Process events, ensuring they are scoped to agent_auth.org_id
        // The filtering above ensures this, but further processing should respect this scope.
    }

    Ok(StatusCode::ACCEPTED)
}

pub async fn ws_agent_ingest(
    ws: WebSocketUpgrade,
    Extension(agent_auth): Extension<AgentAuth>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_agent_ingest_socket(socket, agent_auth))
}

async fn handle_agent_ingest_socket(mut socket: WebSocket, agent_auth: AgentAuth) {
    info!(
        "Agent WebSocket connection established for org_id: {}, key_id: {}...{}",
        agent_auth.org_id,
        &agent_auth.key_id[..4],
        &agent_auth.key_id[agent_auth.key_id.len()-4..]
    );

    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    info!("Received text message from agent (org_id: {}): {:?}", agent_auth.org_id, t);
                    // TODO: Process agent text messages (e.g., batched detection events)
                    // For now, echo back
                    let _ = socket.send(Message::Text(format!("Agent (Org {}): Received: {}", agent_auth.org_id, t).into())).await;
                }
                Message::Binary(b) => {
                    info!("Received binary message from agent (org_id: {}): {:?}", agent_auth.org_id, b);
                    // TODO: Process agent binary messages (e.g., heartbeats or other binary data)
                }
                Message::Ping(p) => {
                    info!("Received ping from agent (org_id: {}): {:?}", agent_auth.org_id, p);
                }
                Message::Pong(p) => {
                    info!("Received pong from agent (org_id: {}): {:?}", agent_auth.org_id, p);
                }
                Message::Close(c) => {
                    info!("Agent WebSocket disconnected for org_id: {}: {:?}", agent_auth.org_id, c);
                    break;
                }
            }
        } else {
            info!("Agent disconnected for org_id: {}", agent_auth.org_id);
            return;
        }
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/batch", post(batch))
        .route("/ws", get(ws_agent_ingest))
}