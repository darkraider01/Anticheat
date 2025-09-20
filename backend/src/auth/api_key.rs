use axum::{
    body::Body,
    extract::State,
    middleware::Next,
    response::Response,
    http::{Request, StatusCode},
};
use serde::{Deserialize, Serialize};
use crate::config::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentAuth {
    pub org_id: String,
    pub agent_id: String,
    pub key_prefix: String,
}

impl AgentAuth {
    pub fn new(org_id: String, agent_id: String, key_prefix: String) -> Self {
        Self {
            org_id,
            agent_id,
            key_prefix,
        }
    }
}

pub async fn api_key_middleware(
    State(app_state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = req.headers()
        .get("X-API-Key")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate API key format and prefix
    if !api_key.starts_with(&app_state.api_key_prefix) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // TODO: In production, look up the org_id and agent_id by API key in database
    // For now, parse from the key format: "org_<org_id>_<agent_id>_<random>"
    let parts: Vec<&str> = api_key.split('_').collect();
    if parts.len() < 4 || parts[0] != "org" {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let org_id = parts.join("_");
    let agent_id = parts.join("_");
    
    let agent_auth = AgentAuth::new(org_id, agent_id, app_state.api_key_prefix.clone());
    req.extensions_mut().insert(agent_auth);

    Ok(next.run(req).await)
}
