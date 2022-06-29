use std::{net::SocketAddr};

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_server::Handle;

/// Sample application demonstrating open WebSockets don't contribute to the
/// connection count and don't have an opportunity to shutdown gracefully.
#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(ws_handler));

    let handle = Handle::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("server shut down")
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(websocket_reader)
}

async fn websocket_reader(mut socket: WebSocket) {
    while let Some(Ok(_)) = socket.recv().await {
    }
}
