/*
    Appellation: worker <module>
    Contrib: @FL03
*/
use crate::BodyResponse;
use tower_service::Service;
use worker::{Context, Env, HttpRequest};

/// the primary entry point for the worker
#[worker::event(fetch)]
pub async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> worker::Result<BodyResponse> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(crate::api().call(req).await?)
}
