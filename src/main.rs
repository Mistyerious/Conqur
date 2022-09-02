use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::Response;
use axum::Router;
use axum::routing::get;
use serde::{Serialize,Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "t", content = "d")]
enum WebMessage {
    MessageCreate { message: String, message_id: u64 },
    MessageDelete { message_id: u64 }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ws", get(websocket_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(msg) => msg,
            // Client disconnected so just return
            Err(_) => return,
        };

        let parsed = match serde_json::from_slice::<WebMessage>(&msg.into_data()) {
            Ok(parsed) => {
                println!("{:#?}", parsed);
                parsed
            },
            Err(error) => {
                socket.close().await.unwrap();
                println!("{}\n", error);
                return;
            }
        };

        if socket.send(Message::Binary(serde_json::to_vec(&parsed).unwrap())).await.is_err() {
            return;
        }
    }
}
