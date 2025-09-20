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

#[derive(Debug, Deserialize, ToSchema)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[utoipa::path(
    post,
    path = "/auth/forgot-password",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset email sent"),
        (status = 404, description = "User not found")
    )
)]
pub async fn forgot_password(Json(payload): Json<ForgotPasswordRequest>) -> StatusCode {
    tracing::info!("Forgot password attempt for email: {}", payload.email);
    // TODO: Implement actual forgot password logic (e.g., send email with reset link)
    if payload.email == "test@example.com" {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
 
pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/forgot-password", post(forgot_password))
}