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
    let mut app = sdk::app::Platform::new();

    // declare the network address
    let addr = "0.0.0.0:8080".parse::<core::net::SocketAddr>().unwrap();
    tracing::info!("Listening on {}", addr);
    // initialize the runtime
    let _rt = app.runner(addr)?;
    // log the exit event
    let _ = tracing::info!("successfully exited the application...");
    Ok(())
}

