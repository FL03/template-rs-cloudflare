/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Message;
use axum::{Json, Router};
use axum::routing::get;

/// the base router for the api;
pub fn router() -> Router {
    Router::new().route("/", get(root)).route("/docs", get(docs))
}

async fn root() -> Json<Message> {
    let message = Message::from_message("Hello, World!");
    worker::console_log!("{message}");
    Json(message)
}


async fn docs() -> Json<Message> {
    let message = Message::from_message("Documentation");
    worker::console_log!("{message}");
    Json(message)
}