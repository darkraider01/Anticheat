use axum::Router;
use tower_http::trace::TraceLayer;
use tokio::net::TcpListener;
use std::net::SocketAddr;

use anticheat::{router, telemetry};

#[tokio::main]
async fn main() {
    telemetry::init();

    let app = Router::new()
        .merge(router::create_router())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
