use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade}
    },
    response::IntoResponse,
    routing::get,
    Router,
};
// No futures::stream::StreamExt import needed as it's not directly used here
use tracing::info;

pub async fn ws_dashboard(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    info!("Client sent text: {:?}", t);
                    socket.send(Message::Text(format!("You said: {}", t).into())).await.unwrap();
                }
                Message::Binary(b) => {
                    info!("Client sent binary: {:?}", b);
                }
                Message::Ping(p) => {
                    info!("Client sent ping: {:?}", p);
                }
                Message::Pong(p) => {
                    info!("Client sent pong: {:?}", p);
                }
                Message::Close(c) => {
                    info!("Client sent close: {:?}", c);
                    break;
                }
            }
        } else {
            // client disconnected
            return;
        }
    }
}

pub fn routes() -> Router {
    Router::new().route("/dashboard", get(ws_dashboard))
}