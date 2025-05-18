/*
    Appellation: routes <module>
    Contrib: @FL03
*/
/// this module implements the base routes for the api
pub mod index;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::api;
    #[doc(inline)]
    pub use super::index::router as index;
}

use axum::Router;
use tower_http::classify::StatusInRangeAsFailures;
use tower_http::trace::TraceLayer;

/// [`api`] is the primary router for the application; nests the version routers accordingly 
/// and equips the api with various service layers
pub fn api<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .nest_service("/api", v0())
        .layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
        ))
}

/// [`v0`] captures all of the api routes for v0;
pub fn v0() -> Router {
    Router::new()
        .nest_service("/", index::router())
}