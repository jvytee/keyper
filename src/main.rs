use anyhow::{Context, Result};
use axum::{routing::get, Router};
use getopts::Options;
use std::{env, io};
use tokio::net::TcpListener;
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
    let mut opts = Options::new();
    opts.optopt("p", "port", "Port to listen on", "PORT");

    let args: Vec<String> = env::args().collect();
    let matches = opts
        .parse(&args[1..])
        .context("Could not parse arguments")?;

    let port_str = matches.opt_str("p").unwrap_or("3000".to_string());
    let port: u16 = port_str
        .parse()
        .with_context(|| format!("Could not parse argument {port_str} as valid port number"))?;

    info!("Listening for requests on port {port}");
    serve(port).await?;

    Ok(())
}

async fn serve(port: u16) -> io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    let router = Router::new().route("/hello", get(hello));

    axum::serve(listener, router).await
}

async fn hello() -> String {
    "Hello, world!".to_string()
}
