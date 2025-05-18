/*
    Appellation: template-rs-cloudflare <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # template-rs-cloudflare
//!
//!
/// declare macros that are either exported or simply used internally
#[doc(hidden)]
#[macro_use]
pub(crate) mod macros {
    /// a macro to streamline the implementation of various get/set/with methods
    #[macro_use]
    pub mod gsw;
    /// this module enables certain traits to be _privately_ implemented
    #[macro_use]
    pub mod seal;
}

/// re-import commonly used traits, primitives, etc. from the various submodules
#[doc(inline)]
pub use self::{
    app::{ApiContext, ApiSettings, Platform, api},
    error::*,
    primitives::*,
    types::prelude::*,
    utils::prelude::*,
};

pub mod app;
pub mod data;
pub mod error;

pub(crate) mod primitives {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod constants;
    pub mod statics;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::constants::*;
        #[doc(inline)]
        pub use super::statics::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod id;
    pub mod message;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::id::*;
        #[doc(inline)]
        pub use super::message::*;
    }

    pub(crate) mod aliases {
        /// a type alias for a [`Response`](axum::http::Response) type setup with a
        /// [`Body`](axum::body::Body) type
        pub type BodyResponse = axum::http::Response<axum::body::Body>;
        /// a type alias for a numerical timestamp
        pub type Timestamp = i64;
        /// a type alias for all model IDs
        pub type ItemId = String;
        /// a type alias for database-compatible timestamps
        pub type BigInt = i64;

        pub type BigSerial = BigInt;
    }
}

pub mod utils {

    #[doc(inline)]
    pub use self::prelude::*;

    pub mod generate;
    pub mod time;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::generate::*;
        #[doc(inline)]
        pub use super::time::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::app::prelude::*;
    #[doc(no_inline)]
    pub use crate::data::prelude::*;
    #[doc(no_inline)]
    pub use crate::error::*;
    #[doc(no_inline)]
    pub use crate::primitives::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
    #[doc(no_inline)]
    pub use crate::utils::prelude::*;
}

/// the primary entry point for the worker
#[cfg(feature = "cf")]
#[worker::event(fetch)]
pub async fn fetch(
    req: worker::HttpRequest,
    _env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    use tower_service::Service;
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    // create the service future
    let service = crate::api().call(req);
    // return the response
    Ok(service.await?)
}
