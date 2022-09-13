use axum::extract::ws::{Message, WebSocket};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageCreate {
    message: String,
    message_id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDelete {
    message_id: String
}

pub async fn message_create(message: MessageCreate, _socket: &mut WebSocket, _connection: &DatabaseConnection) {
    // This here will handle everything related to creating a message, e.g saving the message to the database and then responding back to the
    // client with the appropriate data
    println!("Message: {:#?}", message)
}

pub async fn message_delete(message: MessageDelete, socket: &mut WebSocket, _connection: &DatabaseConnection) {
    // This here will handle everything related to deleting a message, e.g removing the message from the database and then responding back to the
    // client with the appropriate data

    if socket.send(Message::Binary(serde_json::to_vec(&message).unwrap())).await.is_err() {
       return; // Client already disconnected so no use.
    }

    println!("Message: {:#?}\n", message)
}