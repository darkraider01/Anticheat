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
) -> Result<Json<Vec<Detection>>, axum::http::StatusCode> {
    // RBAC: Only 'admin' and 'viewer' roles can list detections
    if !claims.has_role("admin") && !claims.has_role("viewer") {
        tracing::warn!("User {} with role {} attempted to access detections without proper permissions.", claims.sub, claims.role);
        return Err(axum::http::StatusCode::FORBIDDEN);
    }

    tracing::info!("Listing detections for org_id: {}", claims.org_id);
    let page = page_params.page.unwrap_or(1);
    let per_page = page_params.per_page.unwrap_or(10);

    // TODO: Implement actual data retrieval from a database or service
    // For now, simulate filtering by org_id and pagination
    let mut all_detections = vec![
        Detection {
            id: "det-1".to_string(),
            org_id: "org_a".to_string(),
            severity: "high".to_string(),
            created_at: "2024-01-01T12:00:00Z".to_string(),
        },
        Detection {
            id: "det-2".to_string(),
            org_id: "org_a".to_string(),
            severity: "medium".to_string(),
            created_at: "2024-01-01T12:05:00Z".to_string(),
        },
        Detection {
            id: "det-3".to_string(),
            org_id: "org_b".to_string(),
            severity: "low".to_string(),
            created_at: "2024-01-01T12:10:00Z".to_string(),
        },
    ];

    // Filter by org_id
    all_detections.retain(|d| d.org_id == claims.org_id);

    // Simulate pagination
    let start_index = ((page - 1) * per_page) as usize;
    let end_index = (start_index + per_page) as usize;

    let paginated_detections = all_detections
        .into_iter()
        .skip(start_index)
        .take(per_page as usize)
        .collect();

    Ok(Json(paginated_detections))
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
) -> Result<Json<Vec<Agent>>, axum::http::StatusCode> {
    // RBAC: Only 'admin' role can list agents
    if !claims.has_role("admin") {
        tracing::warn!("User {} with role {} attempted to access agents without proper permissions.", claims.sub, claims.role);
        return Err(axum::http::StatusCode::FORBIDDEN);
    }

    tracing::info!("Listing agents for org_id: {}", claims.org_id);
    let page = page_params.page.unwrap_or(1);
    let per_page = page_params.per_page.unwrap_or(10);

    // TODO: Implement actual data retrieval from a database or service
    // For now, simulate filtering by org_id and pagination
    let mut all_agents = vec![
        Agent {
            id: "agent-1".to_string(),
            org_id: "org_a".to_string(),
            status: "active".to_string(),
            last_heartbeat: "2024-01-01T12:00:00Z".to_string(),
        },
        Agent {
            id: "agent-2".to_string(),
            org_id: "org_a".to_string(),
            status: "inactive".to_string(),
            last_heartbeat: "2024-01-01T11:00:00Z".to_string(),
        },
        Agent {
            id: "agent-3".to_string(),
            org_id: "org_b".to_string(),
            status: "active".to_string(),
            last_heartbeat: "2024-01-01T13:00:00Z".to_string(),
        },
    ];

    // Filter by org_id
    all_agents.retain(|a| a.org_id == claims.org_id);

    // Simulate pagination
    let start_index = ((page - 1) * per_page) as usize;
    let end_index = (start_index + per_page) as usize;

    let paginated_agents = all_agents
        .into_iter()
        .skip(start_index)
        .take(per_page as usize)
        .collect();

    Ok(Json(paginated_agents))
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
) -> Result<Json<Vec<Alert>>, axum::http::StatusCode> {
    // RBAC: Only 'admin' role can list alerts
    if !claims.has_role("admin") {
        tracing::warn!("User {} with role {} attempted to access alerts without proper permissions.", claims.sub, claims.role);
        return Err(axum::http::StatusCode::FORBIDDEN);
    }

    tracing::info!("Listing alerts for org_id: {}", claims.org_id);
    let page = page_params.page.unwrap_or(1);
    let per_page = page_params.per_page.unwrap_or(10);

    // TODO: Implement actual data retrieval from a database or service
    // For now, simulate filtering by org_id and pagination
    let mut all_alerts = vec![
        Alert {
            id: "alert-1".to_string(),
            org_id: "org_a".to_string(),
            severity: "critical".to_string(),
            created_at: "2024-01-01T12:00:00Z".to_string(),
        },
        Alert {
            id: "alert-2".to_string(),
            org_id: "org_a".to_string(),
            severity: "high".to_string(),
            created_at: "2024-01-01T12:30:00Z".to_string(),
        },
        Alert {
            id: "alert-3".to_string(),
            org_id: "org_b".to_string(),
            severity: "medium".to_string(),
            created_at: "2024-01-01T13:00:00Z".to_string(),
        },
    ];

    // Filter by org_id
    all_alerts.retain(|a| a.org_id == claims.org_id);

    // Simulate pagination
    let start_index = ((page - 1) * per_page) as usize;
    let end_index = (start_index + per_page) as usize;

    let paginated_alerts = all_alerts
        .into_iter()
        .skip(start_index)
        .take(per_page as usize)
        .collect();

    Ok(Json(paginated_alerts))
}

pub fn routes() -> Router {
    Router::new()
        .route("/detections", get(list_detections))
        .route("/agents", get(list_agents))
        .route("/alerts", get(list_alerts))
}