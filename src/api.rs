pub mod authorization;

use authorization::authorization_endpoint;
use axum::extract::State;
use axum::response::Response;
use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use std::io;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::client::TestClientFactory;
use crate::token::{AccessTokenErrorResponse, AccessTokenRequest, AccessTokenResponse};

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

fn token_endpoint(
    State(router_state): State<Arc<RouterState>>,
    Query(access_token_request): Query<AccessTokenRequest>,
) -> Result<AccessTokenResponse, AccessTokenErrorResponse> {
    todo!()
}

impl IntoResponse for AccessTokenResponse {
    fn into_response(self) -> Response {
        todo!()
    }
}

impl IntoResponse for AccessTokenErrorResponse {
    fn into_response(self) -> Response {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        extract::{Query, State},
        http::{HeaderMap, HeaderValue, StatusCode},
    };

    use crate::{
        api::{authorization_endpoint, create_router, index, RouterState},
        authorization::{AuthorizationRequest, ResponseType},
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
