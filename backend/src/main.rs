use axum::Router;
use axum_extra::extract::cookie::Key;
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use tower::ServiceBuilder;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use anticheat::{router, telemetry, config::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    telemetry::init();

    // Load configuration - fail fast if required env vars missing
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable must be set");
    
    let api_key_prefix = std::env::var("API_KEY_PREFIX")
        .unwrap_or_else(|_| "org".to_string());

    // Generate or load cookie secret securely
    let cookie_secret = if cfg!(debug_assertions) {
        // Development only - generate random key
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        std::process::id().hash(&mut hasher);
        std::time::SystemTime::now().hash(&mut hasher);
        format!("{:016x}{:016x}{:016x}{:016x}", hasher.finish(), hasher.finish(), hasher.finish(), hasher.finish())
    } else {
        std::env::var("COOKIE_SECRET")
            .expect("COOKIE_SECRET environment variable must be set in production")
    };

    // Validate cookie secret length
    if cookie_secret.len() < 64 {
        panic!("Cookie secret must be at least 64 characters long");
    }

    let cookie_key = Key::from(cookie_secret.as_bytes());
    let app_state = AppState::new(cookie_key, jwt_secret, api_key_prefix);

    // Secure CORS configuration
    let allowed_origins = [
        "http://localhost:3000".parse()?,
        "https://app.cluelyguard.com".parse()?,
    ];
    
    let cors = CorsLayer::new()
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
        ])
        .allow_origin(allowed_origins)
        .allow_headers([
            axum::http::HeaderName::from_static("content-type"),
            axum::http::HeaderName::from_static("authorization"),
            axum::http::HeaderName::from_static("x-api-key"),
        ])
        .allow_credentials(true);

    let app = Router::new()
        .merge(router::create_router(app_state))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
