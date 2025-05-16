/*
    Appellation: routes <module>
    Contrib: @FL03
*/
pub mod index;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::index::router as index;
    #[doc(inline)]
    pub use super::router as api;
}

// use tower_http::{classify::StatusInRangeAsFailures, trace::TraceLayer};

/// [router] is the primary router for the application;
pub fn router<S>() -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    axum::Router::new().nest_service("/", index::router())
    // .layer(TraceLayer::new(
    //     StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
    // ))
}
