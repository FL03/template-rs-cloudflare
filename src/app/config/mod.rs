/*
    Appellation: config <module>
    Contrib: @FL03
*/

#[doc(inline)]
pub use self::{context::ApiContext, services::*, settings::ApiSettings, types::prelude::*};

pub mod context;
pub(crate) mod settings;

pub mod services {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod database;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::database::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod db_uri;
    pub mod endpoint;
    pub mod mode;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::db_uri::*;
        #[doc(inline)]
        pub use super::endpoint::*;
        #[doc(inline)]
        pub use super::mode::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::context::*;
    #[doc(inline)]
    pub use super::services::prelude::*;
    #[doc(inline)]
    pub use super::settings::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
}
