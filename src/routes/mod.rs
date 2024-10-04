/*
    Appellation: routes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod index;

use axum::Router;

pub fn app() -> Router {
    Router::new().nest_service("/", index::router())
}
