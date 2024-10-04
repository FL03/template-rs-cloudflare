/*
    Appellation: template-rs-cloudflare <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use axum::{body, http, routing::get};
use tower_service::Service;
use worker::*;

#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    _env: worker::Env,
    _ctx: worker::Context,
) -> Result<http::Response<body::Body>> {
    #[cfg(target_family = "wasm")]
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

fn router() -> axum::Router {
    axum::Router::new().route("/", get(root))
}

pub async fn root() -> &'static str {
    let message = Message::new("Hello, World!");
    worker::console_log!("{message}");
    "Hello, World!"
}


#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Message {
    message: String,
}

impl Message {
    pub fn new(message: impl ToString) -> Self {
        Self { message: message.to_string() }
    }
}

impl core::fmt::Debug for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string_pretty(self).unwrap().as_str())
    }
}

impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}