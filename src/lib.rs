/*
    Appellation: template-rs-cloudflare <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # template-rs-cloudflare
//!
//!
/// declare macros that are either exported or simply used internally
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod gsw;
    #[macro_use]
    pub(crate) mod seal;
}

#[cfg(feature = "cf")]
pub use self::app::worker::*;

#[doc(inline)]
pub use self::{
    app::{ApiContext, ApiSettings, api},
    error::*,
    types::prelude::*,
    utils::prelude::*,
};

pub mod app;
pub mod data;
pub mod error;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod message;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::message::*;
    }

    pub(crate) mod aliases {
        /// a type alias for a [`Response`](axum::http::Response) type setup with a
        /// [`Body`](axum::body::Body) type
        pub type BodyResponse = axum::http::Response<axum::body::Body>;
        /// a type alias for a numerical timestamp
        pub type Timestamp = u128;
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
    pub use crate::types::prelude::*;
    #[doc(no_inline)]
    pub use crate::utils::prelude::*;
}

