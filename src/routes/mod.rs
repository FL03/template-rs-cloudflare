/*
    Appellation: routes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod index;

use axum::Router;

// use tower_http::{classify::StatusInRangeAsFailures, trace::TraceLayer};

/// [app] is the primary router for the application;
///
///
pub fn app<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().nest_service("/", index::router())
    // .layer(TraceLayer::new(
    //     StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
    // ))
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct AppCtx {}

unsafe impl Send for AppCtx {}

unsafe impl Sync for AppCtx {}
