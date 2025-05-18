/*
    Appellation: interface <module>
    Contrib: @FL03
*/
mod impl_inner;
mod impl_interface;
#[cfg(feature = "native")]
mod impl_native;

use super::config::ApiContext;
use std::sync::Arc;

/// [`Platform`] implements an abstract layer defining the primary interface for the
/// application in the process.
pub struct Platform {
    pub(crate) inner: Arc<PlatformInner>,
}

#[derive(Clone, Debug, Default)]
pub struct PlatformInner {
    pub(crate) ctx: ApiContext,
}

impl Clone for Platform {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for Platform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl core::ops::Deref for Platform {
    type Target = PlatformInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.inner)
    }
}
