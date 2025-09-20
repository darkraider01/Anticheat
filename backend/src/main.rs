use anticheat::config::config::config;
use anticheat::router::create_router;
use anticheat::telemetry;
use axum::http::{Method, HeaderName};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use axum_extra::extract::cookie::Key;

#[tokio::main]
async fn main() {
    telemetry::init();

    let configuration = config().expect("Failed to read configuration.");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Failed to bind port");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any)
        .expose_headers([
            HeaderName::from_static("cookie"),
            HeaderName::from_static("set-cookie"),
            HeaderName::from_static("x-real-ip"),
            HeaderName::from_static("x-forwarded-for"),
            HeaderName::from_static("x-forwarded-proto"),
        ]);

    let cookie_secret = std::env::var("COOKIE_SECRET")
        .unwrap_or_else(|_| "super-secret-cookie-key-that-should-be-32-bytes-long".to_string());
    let app_state = anticheat::config::AppState {
        cookie_key: Key::from(cookie_secret.as_bytes()),
    };

    let app = create_router(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    axum::serve(listener, app.into_make_service()).await.unwrap();
}