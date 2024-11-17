use axum::extract::State;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::io;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::authorization::{
    self, AuthorizationErrorResponse, AuthorizationRequest, AuthorizationResponse,
};
use crate::client::TestClientFactory;
use crate::token::{AccessTokenErrorResponse, AccessTokenRequest, AccessTokenResponse};

pub struct RouterState {
    pub client_factory: TestClientFactory,
}

pub fn create_router(state: RouterState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/authorization", get(authorization_endpoint))
        .route("/token", get(token_endpoint))
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

async fn authorization_endpoint(
    State(router_state): State<Arc<RouterState>>,
    Query(auth_request): Query<AuthorizationRequest>,
) -> Result<AuthorizationResponse, AuthorizationErrorResponse> {
    authorization::authorization_code(auth_request, &router_state.client_factory).await
}

impl IntoResponse for AuthorizationResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl IntoResponse for AuthorizationErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self.error {
            authorization::AuthorizationError::InvalidRequest => StatusCode::BAD_REQUEST,
            authorization::AuthorizationError::UnauthorizedClient => StatusCode::UNAUTHORIZED,
            authorization::AuthorizationError::AccessDenied => StatusCode::FORBIDDEN,
            authorization::AuthorizationError::UnsupportedResponseType => StatusCode::BAD_REQUEST,
            authorization::AuthorizationError::InvalidScope => StatusCode::BAD_REQUEST,
            authorization::AuthorizationError::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            authorization::AuthorizationError::TemporarilyUnavailable => {
                StatusCode::SERVICE_UNAVAILABLE
            }
        };

        (status_code, Json(self)).into_response()
    }
}

async fn token_endpoint(
    Query(access_token_request): Query<AccessTokenRequest>,
) -> Result<AccessTokenResponse, AccessTokenErrorResponse> {
    todo!()
}

impl IntoResponse for AccessTokenResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

impl IntoResponse for AccessTokenErrorResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::extract::{Query, State};

    use crate::{
        authorization::{AuthorizationRequest, ResponseType},
        client::TestClientFactory,
        router::{authorization_endpoint, create_router, index, RouterState},
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

    #[tokio::test]
    async fn test_authorization_endpoint() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "foobar".to_string(),
            state: Some("xyz".to_string()),
            redirect_uri: Some("https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()),
            scope: None,
        };

        let client_factory = TestClientFactory {
            client_ids: vec!["foobar".to_string()],
        };
        let router_state = RouterState { client_factory };

        let response =
            authorization_endpoint(State(Arc::new(router_state)), Query(request.clone()))
                .await
                .unwrap();

        assert_eq!(response.state, request.state);
    }
}
