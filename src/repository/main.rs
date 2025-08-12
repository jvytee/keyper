mod api;
mod cli;
mod core;
mod repositories;

use anyhow::Result;
use api::RouterState;
use repositories::client::TestClientRepository;
use std::{env, process::ExitCode};
use tracing::{error, info};

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    match run().await {
        Ok(()) => {
            info!("Done.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            error!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<()> {
    info!("Parsing command line arguments");
    let args: Vec<String> = env::args().collect();
    let params = cli::parse_args(&args)?;

    if let Some(help) = params.help {
        println!("{help}");
        return Ok(());
    }

    info!("Creating client factory");
    let client_factory = TestClientRepository {
        client_ids: vec!["foobar".to_string()],
    };

    info!("Creating template engine");
    let template_engine = api::create_template_engine()?;

    info!("Creating router");
    let router_state = RouterState {
        client_store: client_factory,
        template_engine,
    };
    let router = api::create_router(router_state);

    info!("Listening for requests on port {}", params.port);
    api::serve(router, params.port).await?;

    Ok(())
}
