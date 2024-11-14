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

struct Params {
    port: u16,
}

async fn run() -> Result<()> {
    info!("Parsing command line arguments");
    let args: Vec<String> = env::args().collect();
    let params = parse_args(&args)?;

    info!("Creating router");
    let router = create_router();

    info!("Listening for requests on port {}", params.port);
    serve(router, params.port).await?;

    Ok(())
}

fn parse_args(args: &[String]) -> Result<Params> {
    let mut opts = Options::new();
    opts.optopt("p", "port", "Port to listen on", "PORT");

    let matches = opts
        .parse(&args[1..])
        .context("Could not parse arguments")?;

    let port_str = matches.opt_str("p").unwrap_or("3000".to_string());
    let port: u16 = port_str
        .parse()
        .with_context(|| format!("Could not parse argument {port_str} as valid port number"))?;

    Ok(Params { port })
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/auth", get(auth))
        .route("/token", get(token))
}

async fn index() -> String {
    "Hello, world!".to_string()
}

async fn auth() -> String {
    "Authorization endpoint".to_string()
}

async fn token() -> String {
    "Token endpoint".to_string()
}

async fn serve(router: Router, port: u16) -> io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, router).await
}

#[cfg(test)]
mod tests {
    use crate::{auth, create_router, index, parse_args, token};

    #[test]
    fn test_parse_args() {
        let args = vec!["keyper".to_string(), "-p".to_string(), "1337".to_string()];
        let params = parse_args(&args).unwrap();
        assert_eq!(params.port, 1337u16);
    }

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

    #[tokio::test]
    async fn test_auth() {
        let response = auth().await;
        assert_eq!(response, "Authorization endpoint");
    }

    #[tokio::test]
    async fn test_token() {
        let response = token().await;
        assert_eq!(response, "Token endpoint");
    }
}
