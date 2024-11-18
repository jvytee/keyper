mod api;
mod cli;
mod core;
mod data;

use anyhow::Result;
use data::client::TestClientFactory;
use api::RouterState;
use std::{env, process::ExitCode};
use tracing::info;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    match run().await {
        Ok(()) => {
            tracing::info!("Done.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            tracing::error!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<()> {
    info!("Parsing command line arguments");
    let args: Vec<String> = env::args().collect();
    let params = cli::parse_args(&args)?;

    info!("Creating client factory");
    let client_factory = TestClientFactory {
        client_ids: vec!["foobar".to_string()],
    };

    info!("Creating router");
    let router_state = RouterState { client_factory };
    let router = api::create_router(router_state);

    info!("Listening for requests on port {}", params.port);
    api::serve(router, params.port).await?;

    Ok(())
}
