/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/


use std::sync::Arc;
#[tracing::instrument(skip_all, name = "shutdown", target = "app")]
pub async fn shutdown(ctx: Arc<crate::Context>) {
    tokio::signal::ctrl_c()
        .await
        .expect("CTRL+C: shutdown failed");
    tracing::warn!("Closing the database connection...");
    ctx.db().close().await;
    tracing::warn!("Shutdown the server...");
}