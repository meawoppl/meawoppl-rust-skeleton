use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use shared::WsMessage;
use tokio::sync::mpsc;

/// WebSocket upgrade handler.
pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    // Split the socket into independent send/receive halves.
    // This lets us send from a spawned task while receiving in the main loop,
    // which is the pattern used across cc-proxy and inboxnegative.
    let (mut sender, mut receiver) = socket.split();

    // Channel for sending messages to the client from anywhere.
    let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();

    // Spawn a task that forwards channel messages to the WebSocket sender.
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Send initial heartbeat via the channel.
    let _ = tx.send(WsMessage::Heartbeat);

    // Receive loop — process incoming messages.
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Deserialize into your shared protocol type.
                match serde_json::from_str::<WsMessage>(&text) {
                    Ok(ws_msg) => {
                        // Handle the message — echo it back as an example.
                        // Replace this with your application logic.
                        let _ = tx.send(ws_msg);
                    }
                    Err(_) => {
                        let _ = tx.send(WsMessage::Error {
                            message: "Invalid message format".to_string(),
                        });
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }

    // Clean up the send task when the receive loop exits.
    send_task.abort();
}
