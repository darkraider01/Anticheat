use axum::{
    body::Body,
    middleware::Next,
    response::Response,
    http::{Request, StatusCode},
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, EncodingKey, Header};
use once_cell::sync::Lazy;

const JWT_SECRET: &[u8] = b"your-secret-key"; // TODO: Load from environment variable

static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_secret(JWT_SECRET));
static ENCODING_KEY: Lazy<EncodingKey> = Lazy::new(|| EncodingKey::from_secret(JWT_SECRET));

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

pub async fn jwt_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let token = if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            header.trim_start_matches("Bearer ").to_string()
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let claims = decode::<Claims>(&token, &DECODING_KEY, &Validation::new(Algorithm::HS512))
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .claims;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}