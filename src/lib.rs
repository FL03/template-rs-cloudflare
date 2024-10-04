/*
    Appellation: template-rs-cloudflare <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//!
//! 
//! 
#[doc(inline)]
pub use self::types::prelude::*;

pub mod routes;
pub mod types;


use worker::{Context, Env, Result};


#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<BodyResponse> {
    use tower_service::Service;

    #[cfg(target_family = "wasm")]
    console_error_panic_hook::set_once();
    Ok(routes::app().call(req).await?)
}


