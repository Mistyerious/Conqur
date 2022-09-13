#![allow(unused_imports)]

extern crate core;

use std::sync::Arc;

use crate::handlers::message::*;
use crate::handlers::user::*;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::middleware::AddExtension;
use axum::response::Response;
use axum::routing::{get, post};
use axum::Extension;
use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;

mod handlers;
mod models;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "t", content = "d")]
enum WebMessage {
    MessageCreate(MessageCreate),
    MessageDelete(MessageDelete),
}

#[derive(Debug, Deserialize)]
struct Config {
    database_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let r = std::fs::File::open("config.yaml").unwrap();
    let yaml: Config = serde_yaml::from_reader(r).unwrap();

    // let connection: DatabaseConnection = sea_orm::Database::connect(&yaml.database_url).await?;
    let mut opt = ConnectOptions::new(yaml.database_url.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true);
    let connection = Database::connect(opt).await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/create", post(create_user))
        .route("/ws", get(websocket_handler))
        .layer(ServiceBuilder::new().layer(Extension(connection)));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

        Ok(())
}

async fn websocket_handler(ws: WebSocketUpgrade, Extension(connection): Extension<DatabaseConnection>) -> Response {
    ws.on_upgrade(move | socket | handle_socket(socket, connection))
}

async fn handle_socket(mut socket: WebSocket, refconnection: DatabaseConnection) {
    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(msg) => msg,
            // Client disconnected so just return
            Err(_) => return,
        };

        match serde_json::from_slice::<WebMessage>(&msg.into_data()) {
            Ok(parsed) => match parsed {
                WebMessage::MessageCreate(message) => message_create(message, &mut socket, connection).await,
                WebMessage::MessageDelete(message) => message_delete(message, &mut socket, connection).await,
            },
            Err(error) => {
                if socket.send(Message::Text(error.to_string())).await.is_err() {
                    return;
                };
                socket.close().await.unwrap();
                return;
            }
        };
    }
}
