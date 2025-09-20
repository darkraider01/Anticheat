use axum::{
    extract::{Query, State, Extension},
    routing::{get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{auth::jwt::Claims, config::AppState}; // Import AppState

#[derive(Debug, Deserialize, ToSchema)]
pub struct PageParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Detection {
    pub id: String,
    pub agent_id: String,
    pub org_id: String,
    pub timestamp: String,
    pub event_type: String,
    pub severity: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Agent {
    pub id: String,
    pub org_id: String,
    pub hostname: String,
    pub last_seen: String,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Alert {
    pub id: String,
    pub org_id: String,
    pub detection_ids: Vec<String>,
    pub timestamp: String,
    pub severity: String,
    pub status: String,
    pub description: String,
}

#[utoipa::path(
    get,
    path = "/v1/detections",
    responses(
        (status = 200, description = "List of detections", body = [Detection])
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn list_detections(
    State(_app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<PageParams>,
) -> Json<Vec<Detection>> {
    tracing::info!("User {} from org {} requested detections", claims.sub, claims.org_id);

    // RBAC: Only admin or viewer roles can list detections
    if claims.role != "admin" && claims.role != "viewer" {
        tracing::warn!("User {} with role {} attempted to access detections", claims.sub, claims.role);
        return Json(vec![]); // Return empty for unauthorized
    }

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    // Simulate fetching detections for the given org_id with pagination
    let mut detections = vec![];
    for i in 0..20 { // Simulate 20 detections
        if i >= (page - 1) * per_page && i < page * per_page {
            detections.push(Detection {
                id: format!("det-{}", i),
                agent_id: "agent-123".to_string(),
                org_id: claims.org_id.clone(),
                timestamp: "2023-01-01T12:00:00Z".to_string(),
                event_type: "malware_detected".to_string(),
                severity: "high".to_string(),
                data: serde_json::json!({"file": "/tmp/malware.exe"}),
            });
        }
    }
    Json(detections)
}

#[utoipa::path(
    get,
    path = "/v1/agents",
    responses(
        (status = 200, description = "List of agents", body = [Agent])
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn list_agents(
    State(_app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<PageParams>,
) -> Json<Vec<Agent>> {
    tracing::info!("User {} from org {} requested agents", claims.sub, claims.org_id);

    // RBAC: Only admin roles can list agents
    if claims.role != "admin" {
        tracing::warn!("User {} with role {} attempted to access agents", claims.sub, claims.role);
        return Json(vec![]); // Return empty for unauthorized
    }

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    // Simulate fetching agents for the given org_id with pagination
    let mut agents = vec![];
    for i in 0..5 { // Simulate 5 agents
        if i >= (page - 1) * per_page && i < page * per_page {
            agents.push(Agent {
                id: format!("agent-{}", i),
                org_id: claims.org_id.clone(),
                hostname: format!("host-{}", i),
                last_seen: "2023-01-01T12:00:00Z".to_string(),
                status: "online".to_string(),
            });
        }
    }
    Json(agents)
}

#[utoipa::path(
    get,
    path = "/v1/alerts",
    responses(
        (status = 200, description = "List of alerts", body = [Alert])
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn list_alerts(
    State(_app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<PageParams>,
) -> Json<Vec<Alert>> {
    tracing::info!("User {} from org {} requested alerts", claims.sub, claims.org_id);

    // RBAC: Only admin roles can list alerts
    if claims.role != "admin" {
        tracing::warn!("User {} with role {} attempted to access alerts", claims.sub, claims.role);
        return Json(vec![]); // Return empty for unauthorized
    }

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    // Simulate fetching alerts for the given org_id with pagination
    let mut alerts = vec![];
    for i in 0..10 { // Simulate 10 alerts
        if i >= (page - 1) * per_page && i < page * per_page {
            alerts.push(Alert {
                id: format!("alert-{}", i),
                org_id: claims.org_id.clone(),
                detection_ids: vec![format!("det-{}", i * 2)],
                timestamp: "2023-01-01T12:00:00Z".to_string(),
                severity: "critical".to_string(),
                status: "open".to_string(),
                description: format!("Malware alert for agent-{}", i),
            });
        }
    }
    Json(alerts)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/detections", get(list_detections))
        .route("/agents", get(list_agents))
        .route("/alerts", get(list_alerts))
}