use axum::{
    routing::post,
    Json,
    Router,
    response::IntoResponse,
    http::StatusCode,
    extract::State,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};

use crate::{auth::jwt::Claims, config::AppState};

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub message: String,
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
pub async fn login(
    State(_app_state): State<AppState>,
    jar: PrivateCookieJar,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    tracing::info!("Login attempt for email: {}", payload.email);
    // TODO: Implement actual authentication logic
    if payload.email == "test@example.com" && payload.password == "password" {
        let claims = Claims::new(
            payload.email.clone(),
            "org_a".to_string(), // Stub org_id
            "admin".to_string(), // Stub role
        );

        let token = claims.encode().expect("Failed to encode JWT");

        let mut cookie = Cookie::new("jwt_token", token);
        cookie.set_http_only(true);
        cookie.set_path("/");

        let response = Json(LoginResponse {
            message: "Login successful".to_string(),
        }).into_response();
        // Use jar.add() as PrivateCookieJar can infer the Key from AppState
        (jar.add(cookie), response).into_response()
    } else {
        (jar, Json(LoginResponse {
            message: "Invalid credentials".to_string(),
        }).into_response()).into_response()
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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/forgot-password", post(forgot_password))
        // The AppState comes from the main router, not created here
        // .with_state(AppState { cookie_key: crate::auth::jwt::COOKIE_SIGNING_KEY.clone() })
}