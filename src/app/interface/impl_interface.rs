/*
    Appellation: impl_interface <module>
    Contrib: @FL03
*/
use crate::app::interface::{Platform, PlatformInner};
use std::sync::Arc;

impl Platform {
    /// create a new [`AppInterface`] instance
    pub fn new() -> Self {
        Self {
            inner: Arc::new(PlatformInner::new()),
        }
    }
}
