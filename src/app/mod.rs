/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    config::{ApiContext, ApiSettings},
    error::AppError,
    interface::Platform,
    routes::api,
    state::AppState,
};

pub mod config;
pub mod error;
pub mod interface;
pub mod routes;
pub mod state;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::config::prelude::*;
    #[doc(inline)]
    pub use super::error::*;
    #[doc(inline)]
    pub use super::routes::prelude::*;
    #[doc(inline)]
    pub use super::state::*;
}
