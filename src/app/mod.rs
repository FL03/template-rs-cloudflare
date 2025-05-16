/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod config;
pub mod routes;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::config::prelude::*;
    #[doc(inline)]
    pub use super::routes::prelude::*;
}

