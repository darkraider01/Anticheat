use axum::{
    extract::{Query, Json, Extension}, Router, routing::get
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::auth::jwt::Claims;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct PageParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PageMeta {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Detection {
    pub id: String,
    pub org_id: String,
    pub severity: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Agent {
    pub id: String,
    pub org_id: String,
    pub status: String,
    pub last_heartbeat: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Alert {
    pub id: String,
    pub org_id: String,
    pub severity: String,
    pub created_at: String,
}

#[utoipa::path(
    get,
    path = "/v1/detections",
    params(
        PageParams
    ),
    responses(
        (status = 200, description = "List of detections", body = Vec<Detection>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn list_detections(
    Extension(claims): Extension<Claims>, // Extracted from JWT middleware
    Query(page_params): Query<PageParams>,
) -> Json<Vec<Detection>> {
    tracing::info!("Listing detections for org_id: {}", claims.org_id);
    let page = page_params.page.unwrap_or(1);
    let per_page = page_params.per_page.unwrap_or(10);
    // TODO: Implement actual pagination and filtering
    Json(vec![
        Detection {
            id: format!("det-{}", page * per_page),
            org_id: claims.org_id.clone(),
            severity: "high".to_string(),
            created_at: "2024-01-01T12:00:00Z".to_string(),
        },
    ])
}

#[utoipa::path(
    get,
    path = "/v1/agents",
    params(
        PageParams
    ),
    responses(
        (status = 200, description = "List of agents", body = Vec<Agent>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn list_agents(
    Extension(claims): Extension<Claims>,
    Query(page_params): Query<PageParams>,
) -> Json<Vec<Agent>> {
    tracing::info!("Listing agents for org_id: {}", claims.org_id);
    let page = page_params.page.unwrap_or(1);
    let per_page = page_params.per_page.unwrap_or(10);
    // TODO: Implement actual pagination
    Json(vec![
        Agent {
            id: format!("agent-{}", page * per_page),
            org_id: claims.org_id.clone(),
            status: "active".to_string(),
            last_heartbeat: "2024-01-01T12:00:00Z".to_string(),
        },
    ])
}

#[utoipa::path(
    get,
    path = "/v1/alerts",
    params(
        PageParams
    ),
    responses(
        (status = 200, description = "List of alerts", body = Vec<Alert>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn list_alerts(
    Extension(claims): Extension<Claims>,
    Query(page_params): Query<PageParams>,
) -> Json<Vec<Alert>> {
    tracing::info!("Listing alerts for org_id: {}", claims.org_id);
    let page = page_params.page.unwrap_or(1);
    let per_page = page_params.per_page.unwrap_or(10);
    // TODO: Implement actual pagination
    Json(vec![
        Alert {
            id: format!("alert-{}", page * per_page),
            org_id: claims.org_id.clone(),
            severity: "critical".to_string(),
            created_at: "2024-01-01T12:00:00Z".to_string(),
        },
    ])
}

pub fn routes() -> Router {
    Router::new()
        .route("/detections", get(list_detections))
        .route("/agents", get(list_agents))
        .route("/alerts", get(list_alerts))
}