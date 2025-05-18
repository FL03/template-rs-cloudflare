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
    #[cfg(feature = "native")]
    pub async fn runner<A>(&mut self, addr: A) -> anyhow::Result<tokio::runtime::Runtime>
    where
        A: tokio::net::ToSocketAddrs,
    {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        // log the event
        #[cfg(feature = "tracing")]
        tracing::trace!("registering the application with the runtime...");
        rt.block_on(self.run(addr))?;
        // wait for the server to finish
        Ok(rt)
    }
    #[cfg(feature = "native")]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self, addr), name = "native", target = "app")
    )]
    pub async fn run<A>(&mut self, addr: A) -> anyhow::Result<()>
    where
        A: tokio::net::ToSocketAddrs,
    {
        async fn graceful_shutdown() {
            // wait for a signal to shutdown
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
            // log the shutdown
            tracing::info!("shutdown signal recieved; terminating services...");
        }

        // initialize the app router
        let app = crate::app::api();
        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        // setup the server
        let server = axum::serve(listener, app).with_graceful_shutdown(graceful_shutdown());
        // wait for the server to finish
        Ok(server.await?)
    }
}

impl AppInner {
    /// create a new [`AppInner`] instance
    pub fn new() -> Self {
        let ctx = ApiContext::default();
        Self::from_ctx(ctx)
    }
    /// create a new [`AppInner`] instance from the given context
    pub fn from_ctx(ctx: ApiContext) -> Self {
        Self {
            ctx: Arc::new(Mutex::new(ctx)),
        }
    }
    /// get a reference to the [`ApiContext`]
    pub fn ctx(&self) -> &Mutex<ApiContext> {
        self.ctx.as_ref()
    }
    /// get a mutable reference to the [`ApiContext`]
    pub fn ctx_mut(&self) -> &Mutex<ApiContext> {
        self.ctx.as_ref()
    }
}
