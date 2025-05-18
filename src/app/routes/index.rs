/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Message;
use axum::response::{Html, Json};
use axum::routing::get;

/// the base router for the api;
pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .route("/docs", get(docs))
        .route("/test", get(test))
}

async fn root() -> String {
    String::from("Hello, World!")
}

async fn test() -> Json<Message> {
    let message = Message::from_message("Hello, World!");
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
