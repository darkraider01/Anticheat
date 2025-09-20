use axum::{
    extract::{Query, Extension},
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::{auth::jwt::Claims, config::AppState};

// Pagination and filtering types
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PaginationParams {
    #[validate(range(min = 1, max = 1000, message = "Page must be between 1 and 1000"))]
    #[serde(default = "default_page")]
    pub page: u32,
    
    #[validate(range(min = 1, max = 100, message = "Per page must be between 1 and 100"))]
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

#[derive(Debug, Serialize, ToSchema)]
pub struct PageMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

// Detection types
#[derive(Debug, Deserialize, Validate, ToSchema, utoipa::IntoParams)]
pub struct DetectionFilters {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    
    pub severity: Option<String>,
    pub agent_id: Option<String>,
    pub detection_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Detection {
    pub id: String,
    pub org_id: String,
    pub agent_id: String,
    pub detection_type: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Agent types
#[derive(Debug, Deserialize, Validate, ToSchema, utoipa::IntoParams)]
pub struct AgentFilters {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    
    pub status: Option<String>,
    pub platform: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Agent {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub platform: String,
    pub version: String,
    pub status: String,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Alert types
#[derive(Debug, Deserialize, Validate, ToSchema, utoipa::IntoParams)]
pub struct AlertFilters {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    
    pub status: Option<String>,
    pub severity: Option<String>,
    pub rule_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Alert {
    pub id: String,
    pub org_id: String,
    pub rule_id: String,
    pub detection_id: String,
    pub severity: String,
    pub status: String,
    pub title: String,
    pub description: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// List detections with org-scoped access
#[utoipa::path(
    get,
    path = "/v1/detections",
    params(DetectionFilters),
    responses(
        (status = 200, description = "Paginated list of detections", body = PagedResponse<Detection>),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid query parameters")
    ),
    security(("bearerAuth" = [])),
    tag = "Detections"
)]
pub async fn list_detections(
    Extension(claims): Extension<Claims>,
    Query(filters): Query<DetectionFilters>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate query parameters
    if let Err(_) = filters.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Enforce org-scoped access - all queries are automatically scoped by org_id from JWT
    let org_id = &claims.org_id;
    
    // TODO: Replace with actual database query
    // For now, return mock data filtered by org_id
    let mock_detections = vec![
        Detection {
            id: "det_001".to_string(),
            org_id: org_id.clone(),
            agent_id: "agent_001".to_string(),
            detection_type: "suspicious_process".to_string(),
            severity: "high".to_string(),
            title: "Suspicious AI Process Detected".to_string(),
            description: "Detected ChatGPT API calls during gameplay".to_string(),
            metadata: serde_json::json!({"process": "chatgpt.exe", "confidence": 0.95}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    ];
    
    let total = mock_detections.len() as u64;
    let total_pages = ((total as f64) / (filters.pagination.per_page as f64)).ceil() as u32;
    
    let response = PagedResponse {
        data: mock_detections,
        meta: PageMeta {
            page: filters.pagination.page,
            per_page: filters.pagination.per_page,
            total,
            total_pages,
        },
    };
    
    Ok((StatusCode::OK, Json(response)))
}

/// List agents with org-scoped access
#[utoipa::path(
    get,
    path = "/v1/agents",
    params(AgentFilters),
    responses(
        (status = 200, description = "Paginated list of agents", body = PagedResponse<Agent>),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid query parameters")
    ),
    security(("bearerAuth" = [])),
    tag = "Agents"
)]
pub async fn list_agents(
    Extension(claims): Extension<Claims>,
    Query(filters): Query<AgentFilters>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(_) = filters.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let org_id = &claims.org_id;
    
    // TODO: Replace with actual database query
    let mock_agents = vec![
        Agent {
            id: "agent_001".to_string(),
            org_id: org_id.clone(),
            name: "Game Server #1".to_string(),
            platform: "Windows".to_string(),
            version: "1.0.0".to_string(),
            status: "online".to_string(),
            last_heartbeat: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    ];
    
    let total = mock_agents.len() as u64;
    let total_pages = ((total as f64) / (filters.pagination.per_page as f64)).ceil() as u32;
    
    let response = PagedResponse {
        data: mock_agents,
        meta: PageMeta {
            page: filters.pagination.page,
            per_page: filters.pagination.per_page,
            total,
            total_pages,
        },
    };
    
    Ok((StatusCode::OK, Json(response)))
}

/// List alerts with org-scoped access
#[utoipa::path(
    get,
    path = "/v1/alerts",
    params(AlertFilters),
    responses(
        (status = 200, description = "Paginated list of alerts", body = PagedResponse<Alert>),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid query parameters")
    ),
    security(("bearerAuth" = [])),
    tag = "Alerts"
)]
pub async fn list_alerts(
    Extension(claims): Extension<Claims>,
    Query(filters): Query<AlertFilters>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(_) = filters.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let org_id = &claims.org_id;
    
    // TODO: Replace with actual database query
    let mock_alerts = vec![
        Alert {
            id: "alert_001".to_string(),
            org_id: org_id.clone(),
            rule_id: "rule_001".to_string(),
            detection_id: "det_001".to_string(),
            severity: "high".to_string(),
            status: "new".to_string(),
            title: "High Severity Detection".to_string(),
            description: "Multiple suspicious processes detected".to_string(),
            metadata: serde_json::json!({"count": 5, "threshold": 3}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    ];
    
    let total = mock_alerts.len() as u64;
    let total_pages = ((total as f64) / (filters.pagination.per_page as f64)).ceil() as u32;
    
    let response = PagedResponse {
        data: mock_alerts,
        meta: PageMeta {
            page: filters.pagination.page,
            per_page: filters.pagination.per_page,
            total,
            total_pages,
        },
    };
    
    Ok((StatusCode::OK, Json(response)))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/detections", axum::routing::get(list_detections))
        .route("/agents", axum::routing::get(list_agents))
        .route("/alerts", axum::routing::get(list_alerts))
}

