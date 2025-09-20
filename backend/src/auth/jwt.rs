use axum::{
    body::Body,
    middleware::Next,
    response::Response,
    http::{Request, StatusCode},
    extract::State,
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, EncodingKey, Header};
use once_cell::sync::Lazy;
use axum_extra::extract::cookie::{PrivateCookieJar};
use std::env;
use tracing::error;

use crate::config::AppState; // Import AppState

static JWT_SECRET_BYTES: Lazy<Vec<u8>> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable not set");
    let bytes = secret.as_bytes().to_vec();
    if bytes.len() < 32 {
        panic!("JWT_SECRET must be at least 32 bytes long for security");
    }
    bytes
});

static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_secret(&JWT_SECRET_BYTES));
static ENCODING_KEY: Lazy<EncodingKey> = Lazy::new(|| EncodingKey::from_secret(&JWT_SECRET_BYTES));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub org_id: String,
    pub role: String,
    pub exp: usize, // Expiration time (as UTC timestamp)
}

impl Claims {
    pub fn new(sub: String, org_id: String, role: String) -> Self {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        Claims {
            sub,
            org_id,
            role,
            exp: expiration,
        }
    }

    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::new(Algorithm::HS512), self, &ENCODING_KEY)
    }

    pub fn has_role(&self, role_name: &str) -> bool {
        self.role == role_name
    }
}

// Removed COOKIE_SIGNING_KEY from here, it's now in AppState
// pub static COOKIE_SIGNING_KEY: Lazy<Key> = Lazy::new(|| {
//     let secret = env::var("COOKIE_SECRET")
//         .unwrap_or_else(|_| "super-secret-cookie-key-that-should-be-32-bytes-long".to_string()); // Fallback for development
//     Key::from(secret.as_bytes())
// });

pub async fn jwt_middleware(
    State(_app_state): State<AppState>, // Accept AppState
    jar: PrivateCookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = jar.get("jwt_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok())
                .and_then(|header| {
                    if header.starts_with("Bearer ") {
                        Some(header.trim_start_matches("Bearer ").to_string())
                    } else {
                        None
                    }
                })
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = decode::<Claims>(
        &token,
        &DECODING_KEY,
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|e| {
        error!("JWT decoding error: {:?}", e);
        StatusCode::UNAUTHORIZED
    })?
    .claims;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}