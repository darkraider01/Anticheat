use axum::{routing::post, Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<LoginResponse>) {
    tracing::info!("Login attempt for email: {}", payload.email);
    // TODO: Implement actual authentication logic
    if payload.email == "test@example.com" && payload.password == "password" {
        (
            StatusCode::OK,
            Json(LoginResponse {
                token: "<stub>".to_string(),
            }),
        )
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                token: "invalid".to_string(),
            }),
        )
    }
}

pub fn routes() -> Router {
    Router::new().route("/login", post(login))
}