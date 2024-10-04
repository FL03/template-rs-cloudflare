/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::message::Message;

pub mod message;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::message::*;
    pub(crate) use super::BodyResponse;
}

pub(crate) type BodyResponse = axum::http::Response<axum::body::Body>;
