pub mod authorization;
pub mod token;

use authorization::authorization_endpoint;
use axum::{routing::get, Router};
use std::io;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::client::TestClientFactory;

#[derive(Debug)]
pub struct RouterState {
    pub client_factory: TestClientFactory,
}

pub fn create_router(state: RouterState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/authorization", get(authorization_endpoint))
        // .route("/token", get(token_endpoint))
        .with_state(Arc::new(state))
}

pub async fn serve(router: Router, port: u16) -> io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, router).await
}

async fn index() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{create_router, index, RouterState},
        client::TestClientFactory,
    };

    #[test]
    fn test_create_router() {
        let client_factory = TestClientFactory {
            client_ids: vec!["foobar".to_string()],
        };
        let router_state = RouterState { client_factory };
        let router = create_router(router_state);

        assert!(router.has_routes());
    }

    #[tokio::test]
    async fn test_index() {
        let response = index().await;
        assert_eq!(response, "Hello, world!");
    }
}
