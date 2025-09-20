use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    body::Body,
    http::StatusCode,
    extract::State,
};
use serde::{Deserialize, Serialize};

use crate::config::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentAuth {
    pub org_id: String,
    pub key_id: String,
}

pub async fn api_key_middleware(
    State(_app_state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = req.headers()
        .get("X-API-Key")
        .and_then(|header| header.to_str().ok());

    if api_key.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // TODO: Look up org_id by key; inject AgentAuth
    // For now, we'll just create dummy AgentAuth
    let agent_auth = AgentAuth {
        org_id: "org789".to_string(),
        key_id: api_key.unwrap().to_string(),
    };

    req.extensions_mut().insert(agent_auth);

    Ok(next.run(req).await)
}