use axum::Json;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use axum::http::StatusCode;

use crate::handlers; // Import handlers module

use crate::handlers::{
    auth::{LoginRequest, LoginResponse},
    dashboard_api::{Agent, Alert, Detection, PageParams},
    ingest::{IngestBatchRequest, IngestEvent},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::healthz,
        handlers::version,
        handlers::auth::login,
        handlers::ingest::batch,
        handlers::dashboard_api::list_detections,
        handlers::dashboard_api::list_agents,
        handlers::dashboard_api::list_alerts,
    ),
    components(
        schemas(
            handlers::HealthzResponse,
            handlers::VersionResponse,
            LoginRequest,
            LoginResponse,
            IngestEvent,
            IngestBatchRequest,
            PageParams,
            Detection,
            Agent,
            Alert,
        )
    ),
    tags(
        (name = "AIGuard", description = "AIGuard API endpoints")
    ),
    modifiers(&SecurityAddon),
    security(
        ("bearerAuth" = []),
        ("apiKeyAuth" = [])
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
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
                SecurityScheme::ApiKey(
                    utoipa::openapi::security::ApiKey::Header(
                        utoipa::openapi::security::ApiKeyValue::new("X-API-Key"),
                    ),
                ),
            );
        }
    }
}

pub async fn serve_openapi() -> impl axum::response::IntoResponse {
    Json(ApiDoc::openapi())
}

pub fn routes() -> impl axum::response::IntoResponse {
    // This function will no longer return SwaggerUi directly.
    // The Swagger UI will not be served by the Rust backend due to dependency conflicts.
    // OpenAPI JSON will still be available at /api-docs/openapi.json
    StatusCode::NOT_FOUND // This was a placeholder, will be replaced once SwaggerUi issue is resolved
}