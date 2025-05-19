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

/// [`api`] is the primary router for the application; nests the version routers accordingly
/// and equips the api with various service layers
pub fn api() -> Router {
    Router::new()
        .merge(v0())
        .layer(tower_http::trace::TraceLayer::new(
            tower_http::classify::StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
        ))
}

/// [`v0`] captures all of the api routes for v0;
pub fn v0() -> Router {
    Router::new().merge(index::router())
}
