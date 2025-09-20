use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade}, Extension
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::StreamExt;
use tracing::info;
use crate::auth::api_key::AgentAuth;

pub async fn ws_dashboard(
    ws: WebSocketUpgrade,
    Extension(agent_auth): Extension<AgentAuth>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, agent_auth))
}

async fn handle_socket(mut socket: WebSocket, agent_auth: AgentAuth) {
    info!("WebSocket connection established for org_id: {}, key_id: {}", agent_auth.org_id, agent_auth.key_id);

    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    info!("Received text message from org_id: {}: {:?}", agent_auth.org_id, t);
                    // For now, just echo back with org_id context
                    socket.send(Message::Text(format!("Org {}: You said: {}", agent_auth.org_id, t).into())).await.unwrap();
                }
                Message::Binary(b) => {
                    info!("Received binary message from org_id: {}: {:?}", agent_auth.org_id, b);
                }
                Message::Ping(p) => {
                    info!("Received ping from org_id: {}: {:?}", agent_auth.org_id, p);
                }
                Message::Pong(p) => {
                    info!("Received pong from org_id: {}: {:?}", agent_auth.org_id, p);
                }
                Message::Close(c) => {
                    info!("WebSocket disconnected for org_id: {}: {:?}", agent_auth.org_id, c);
                    break;
                }
            }
        } else {
            info!("Client disconnected for org_id: {}", agent_auth.org_id);
            return;
        }
    }
}

pub fn routes() -> Router {
    Router::new().route("/dashboard", get(ws_dashboard))
}