/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Message;
use axum::{routing::get, Json};

pub fn router() -> axum::Router {
    axum::Router::new().route("/", get(root))
}

async fn root() -> Json<Message> {
    let message = Message::new("Hello, World!");
    worker::console_log!("{message}");
    Json(message)
}
