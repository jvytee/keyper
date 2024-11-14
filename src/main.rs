mod authorization;
mod cli;
mod token;

use anyhow::Result;
use axum::{routing::get, Router};
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
    info!("Parsing command line arguments");
    let args: Vec<String> = env::args().collect();
    let params = cli::parse_args(&args)?;

    info!("Creating router");
    let router = create_router();

    info!("Listening for requests on port {}", params.port);
    serve(router, params.port).await?;

    Ok(())
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/auth", get(authorization::handler))
        .route("/token", get(token::handler))
}

async fn index() -> String {
    "Hello, world!".to_string()
}

async fn serve(router: Router, port: u16) -> io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, router).await
}

#[cfg(test)]
mod tests {
    use super::{create_router, index};

    #[test]
    fn test_create_router() {
        let router = create_router();
        assert!(router.has_routes());
    }

    #[tokio::test]
    async fn test_index() {
        let response = index().await;
        assert_eq!(response, "Hello, world!");
    }
}
