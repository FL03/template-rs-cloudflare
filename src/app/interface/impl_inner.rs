/*
    Appellation: impl_inner <module>
    Contrib: @FL03
*/
use super::PlatformInner;
use crate::app::config::{ApiContext, ApiSettings};

impl PlatformInner {
    /// create a new [`AppInner`] instance
    pub fn new() -> Self {
        let ctx = ApiContext::default();
        Self::from_ctx(ctx)
    }
    /// create a new [`AppInner`] instance from the given context
    pub fn from_ctx(ctx: ApiContext) -> Self {
        Self { ctx }
    }
    /// returns an immutable reference to the current [`config`](ApiSettings)
    pub const fn config(&self) -> &ApiSettings {
        self.ctx().config()
    }
    /// returns a mutable reference to the current [`config`](ApiSettings)
    pub const fn config_mut(&mut self) -> &mut ApiSettings {
        self.ctx_mut().config_mut()
    }
    /// returns an immutable reference to the application [context](ApiContext)
    pub const fn ctx(&self) -> &ApiContext {
        &self.ctx
    }
    /// returns a mutable reference to the application [context](ApiContext)
    pub const fn ctx_mut(&mut self) -> &mut ApiContext {
        &mut self.ctx
    }
    /// updates the current context then returns a mutable reference to the parent instance
    #[inline]
    pub fn set_ctx(&mut self, ctx: ApiContext) -> &mut Self {
        self.ctx = ctx;
        self
    }
    /// consumes the current instance to create another with the given context
    #[inline]
    pub fn with_ctx(self, ctx: ApiContext) -> Self {
        Self { ctx, ..self }
    }
}
