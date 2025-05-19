/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Message;
use axum::response::Json;
use axum::routing::get;

/// the base router for the api;
pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .route("/test", get(test))
}

/// the root route for the api;
async fn root() -> String {
    String::from("Hello, World!")
}

async fn test() -> Json<Message> {
    let message = Message::from_message("Hello, World!");
    Json(message)
}

