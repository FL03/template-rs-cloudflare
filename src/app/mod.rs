/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{config::Settings, context::Context};

pub mod config;
pub mod context;

#[allow(unused)]
pub(crate) mod prelude {
    pub use super::config::Settings;
    pub use super::context::Context;
}
