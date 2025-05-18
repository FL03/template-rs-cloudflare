/*
    Appellation: main <module>
    Contrib: @FL03
*/

fn main() -> anyhow::Result<()> {
    // initialize the tracer
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // log the event
    tracing::debug!("successfully initialized the tracing modules the application...");
    // log the startup event
    tracing::info!("initializing the application...");
    let mut app = sdk::app::AppInterface::new();

    // declare the network address
    let addr = "0.0.0.0:8080".parse::<core::net::SocketAddr>().unwrap();
    // setup the runtime
    let rt = _runtime()?;
    // register the runner
    let _output = rt.block_on(app.run(addr));
    // log the address
    tracing::info!("Listening on {}", addr);
    if let Err(e) = _output {
        tracing::error!("Error: {}", e);
    }
    tracing::info!("successfully exited the application...");
    Ok(())
}

/// a custom runtime designed for the application
fn _runtime() -> Result<tokio::runtime::Runtime, std::io::Error> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
}

/// the main entry point of the application;
#[tracing::instrument(skip(addr), target = "rscloud")]
async fn _runner<A>(addr: A) -> anyhow::Result<()>
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
    let app = sdk::app::api();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // serve the app
    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown())
        .await?;
    // wait for the server to finish
    Ok(())
}
