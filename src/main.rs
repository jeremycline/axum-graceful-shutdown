use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_server::Handle;
use futures_util::{SinkExt, StreamExt};
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

/// Sample application demonstrating open WebSockets don't contribute to the
/// connection count and don't have an opportunity to shutdown gracefully.
#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(ws_handler));

    let handle = Handle::new();

    tokio::spawn(websocket_client(handle.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("server shut down")
}

async fn graceful_shutdown(handle: Handle) {
    handle.graceful_shutdown(Some(Duration::from_secs(30)));

    loop {
        println!("alive connections: {}", handle.connection_count());
        sleep(Duration::from_secs(1)).await;
    }
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let axum::extract::ws::Message::Text(msg) = msg {
            println!("Got {}", msg)
        }
    }
}

async fn websocket_client(handle: Handle) {
    let url = url::Url::parse("ws://127.0.0.1:3000/ws").unwrap();

    let (ws_stream, _) = connect_async(url).await.unwrap();
    let (mut writer, _reader) = ws_stream.split();

    tokio::spawn(graceful_shutdown(handle.clone()));

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        writer
            .send(Message::Text("beep".to_string()))
            .await
            .unwrap();
    }
}
