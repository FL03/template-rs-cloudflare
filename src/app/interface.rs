
use super::config::ApiContext;
use std::sync::{Arc, Mutex};

pub struct AppInterface {
    pub(crate) inner: Arc<AppInner>,
}

pub struct AppInner {
    pub(crate) ctx: Arc<Mutex<ApiContext>>,
}

impl AppInterface {
    /// create a new [`AppInterface`] instance
    pub fn new() -> Self {
        Self {
            inner: Arc::new(AppInner::new()),
        }
    }
}


