/*
    Appellation: data <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! 
pub mod models {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod items;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::items::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::models::prelude::*;
}
