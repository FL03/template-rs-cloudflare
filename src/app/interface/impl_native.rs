/*
    Appellation: impl_native <module>
    Contrib: @FL03
*/

use crate::app::interface::Platform;
use tokio::net::ToSocketAddrs;

impl Platform {
    pub fn runner<A>(&mut self, addr: A) -> anyhow::Result<tokio::runtime::Runtime>
    where
        A: ToSocketAddrs,
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
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self, addr), name = "native", target = "app")
    )]
    pub async fn run<A>(&mut self, addr: A) -> anyhow::Result<()>
    where
        A: ToSocketAddrs,
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
