use axum::{
    body::Body,
    extract::State,
    middleware::Next,
    response::Response,
    http::{Request, StatusCode},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use crate::config::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub org_id: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: String, org_id: String, role: String) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        Self {
            sub: user_id,
            org_id,
            role,
            iat: now,
            exp: now + Duration::hours(24).whole_seconds(),
        }
    }

    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }
    
    pub fn decode(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims)
    }
}

pub async fn jwt_middleware(
    State(app_state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let token = if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            header.trim_start_matches("Bearer ")
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    match Claims::decode(token, &app_state.jwt_secret) {
        Ok(claims) => {
            // Check if token is expired
            let now = OffsetDateTime::now_utc().unix_timestamp();
            if claims.exp < now {
                return Err(StatusCode::UNAUTHORIZED);
            }
            
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
