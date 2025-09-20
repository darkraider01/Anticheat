use axum::{response::IntoResponse, Json};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::login,
        crate::handlers::ingest::batch_ingest,
        crate::handlers::dashboard_api::list_detections,
        crate::handlers::dashboard_api::list_agents,
        crate::handlers::dashboard_api::list_alerts,
        crate::handlers::healthz,
        crate::handlers::version,
    ),
    components(
        schemas(
            // Auth schemas
            crate::handlers::auth::LoginRequest,
            crate::handlers::auth::LoginResponse,
            crate::handlers::auth::UserInfo,
            
            // Ingest schemas
            crate::handlers::ingest::IngestBatchRequest,
            crate::handlers::ingest::DetectionEvent,
            crate::handlers::ingest::AgentHeartbeat,
            crate::handlers::ingest::IngestResponse,
            
            // Dashboard schemas
            crate::handlers::dashboard_api::PaginationParams,
            crate::handlers::dashboard_api::PageMeta,
            crate::handlers::dashboard_api::Detection,
            crate::handlers::dashboard_api::DetectionFilters,
            crate::handlers::dashboard_api::Agent,
            crate::handlers::dashboard_api::AgentFilters,
            crate::handlers::dashboard_api::Alert,
            crate::handlers::dashboard_api::AlertFilters,
            
            // Common schemas
            crate::handlers::HealthzResponse,
            crate::handlers::VersionResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "User authentication and session management"),
        (name = "Ingest", description = "Agent data ingestion endpoints"),
        (name = "Detections", description = "Detection management and querying"),
        (name = "Agents", description = "Agent fleet management"),
        (name = "Alerts", description = "Alert management and workflow"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme, ApiKeyValue, ApiKey};
        
        let components = openapi.components.as_mut().unwrap();
        
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        
        components.add_security_scheme(
            "apiKeyAuth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key"))),
        );
    }
}

pub async fn serve_openapi() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}
