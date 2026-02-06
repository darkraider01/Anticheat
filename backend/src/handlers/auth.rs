use axum::{
    extract::{State},
    response::{IntoResponse, Response},
    Json,
    http::StatusCode,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::{auth::jwt::Claims, config::AppState};

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<UserInfo>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub org_id: String,
    pub role: String,
}

/// Login endpoint that authenticates users and sets JWT cookie
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 400, description = "Invalid request format")
    ),
    tag = "Authentication"
)]
pub async fn login(
    State(app_state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, Response> {
    // Validate input
    if let Err(validation_errors) = payload.validate() {
        // Log detailed errors for debugging, but return generic message to user
        tracing::warn!("Login validation failed: {:?}", validation_errors);
        return Ok((
            jar,
            (StatusCode::BAD_REQUEST, Json(LoginResponse {
                success: false,
                message: "Invalid request format".to_string(),
                user: None,
            }))
        ));
    }

    // For demo purposes - replace with actual user authentication
    // In production, verify against database with hashed passwords
    if payload.email == "demo@cluelyguard.com" && payload.password == "demo123456" {
        let user_id = uuid::Uuid::new_v4().to_string();
        let org_id = "demo_org_001".to_string();
        let role = "admin".to_string();
        
        let claims = Claims::new(user_id.clone(), org_id.clone(), role.clone());
        
        match claims.encode(&app_state.jwt_secret) {
            Ok(token) => {
                let mut cookie = Cookie::new("jwt_token", token);
                cookie.set_http_only(true);
                cookie.set_path("/");
                cookie.set_secure(true); // Secure flag for HTTPS
                cookie.set_same_site(axum_extra::extract::cookie::SameSite::Strict);
                cookie.set_max_age(time::Duration::hours(24));
                
                let jar = jar.add(cookie);
                
                Ok((
                    jar,
                    (StatusCode::OK, Json(LoginResponse {
                        success: true,
                        message: "Login successful".to_string(),
                        user: Some(UserInfo {
                            id: user_id,
                            email: payload.email,
                            org_id,
                            role,
                        }),
                    }))
                ))
            }
            Err(e) => {
                tracing::error!("Failed to encode JWT: {}", e);
                Ok((jar, (StatusCode::INTERNAL_SERVER_ERROR, Json(LoginResponse {
                    success: false,
                    message: "Authentication failed".to_string(),
                    user: None,
                }))))
            }
        }
    } else {
        Ok((jar, (StatusCode::UNAUTHORIZED, Json(LoginResponse {
            success: false,
            message: "Invalid email or password".to_string(),
            user: None,
        }))))
    }
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/login", axum::routing::post(login))
}
