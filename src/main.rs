use axum::{routing::get, Router};
use std::io::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting keyper...");

    match run().await {
        Ok(()) => tracing::info!("Done."),
        Err(error) => tracing::error!("{}", error)
    };
}

async fn run() -> Result<()> {
    let router = Router::new().route("/hello", get(hello));
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await
}

async fn hello() -> String {
    "Hello, world!".to_string()
}
