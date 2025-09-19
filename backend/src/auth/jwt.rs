use axum::{
    body::Body,
    middleware::Next,
    response::Response,
    http::{Request, StatusCode},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub org_id: String,
    pub role: String,
}

pub async fn jwt_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let _token = if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            header.trim_start_matches("Bearer ").to_string()
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // TODO: Verify JWT token and extract claims
    // For now, we'll just create dummy claims
    let claims = Claims {
        sub: "user123".to_string(),
        org_id: "org456".to_string(),
        role: "admin".to_string(),
    };

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}