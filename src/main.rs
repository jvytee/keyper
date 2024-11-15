mod authorization;
mod cli;
mod router;
mod token;

use anyhow::Result;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    match run().await {
        Ok(()) => tracing::info!("Done."),
        Err(error) => tracing::error!("Error: {}", error),
    };
}

async fn run() -> Result<()> {
    info!("Parsing command line arguments");
    let args: Vec<String> = env::args().collect();
    let params = cli::parse_args(&args)?;

    info!("Creating router");
    let router = router::create_router();

    info!("Listening for requests on port {}", params.port);
    router::serve(router, params.port).await?;

    Ok(())
}
