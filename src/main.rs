mod authorization;
mod cli;
mod client;
mod router;
mod token;
mod user;

use anyhow::Result;
use client::TestClientFactory;
use router::RouterState;
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

    info!("Creating client factory");
    let client_factory = TestClientFactory { client_ids: vec!["foobar".to_string()] };

    info!("Creating router");
    let router_state = RouterState { client_factory };
    let router = router::create_router(router_state);

    info!("Listening for requests on port {}", params.port);
    router::serve(router, params.port).await?;

    Ok(())
}
