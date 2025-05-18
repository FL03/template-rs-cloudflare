/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Message;
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};

/// the base router for the api;
pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/docs", get(docs))
}

async fn root() -> Json<Message> {
    let message = Message::from_message("Hello, World!");
    #[cfg(feature = "cf")]
    worker::console_log!("{message}");
    Json(message)
}

async fn docs() -> Html<String> {
    let doc = r#"
        <html>
            <head>
                <title>Documentation</title>
            </head>
            <body>
                <h1>Docs</h1>
                <span>Welcome to the template-rs-cloudflare documentation!</span>

                <div>
                    <div>
                        <ul>
                            <li><a href="/">Home</a></li>
                            <li><a href="/docs">Docs</a></li>
                        </ul>
                    </div>
                </div>
            </body>
        </html>
        "#;
    return Html(doc.to_string());
}
