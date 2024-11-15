use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::io;
use tokio::net::TcpListener;

use crate::{
    authorization::{
        self, AuthorizationErrorResponse, AuthorizationRequest, AuthorizationResponse,
    },
    token,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/authorization", get(authorization_endpoint))
        .route("/token", get(token::handler))
}

async fn index() -> String {
    "Hello, world!".to_string()
}

pub async fn serve(router: Router, port: u16) -> io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, router).await
}

pub async fn authorization_endpoint(
    Query(auth_request): Query<AuthorizationRequest>,
) -> Result<AuthorizationResponse, AuthorizationErrorResponse> {
    authorization::authorization_code(auth_request).await
}

impl IntoResponse for AuthorizationResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl IntoResponse for AuthorizationErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::UNAUTHORIZED, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::extract::Query;

    use crate::{
        authorization::{AuthorizationRequest, ResponseType},
        router::{authorization_endpoint, create_router, index},
    };

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
    async fn test_handler() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "s6BhdRkqt3".to_string(),
            state: Some("xyz".to_string()),
            redirect_uri: Some("https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()),
            scope: None,
        };

        let response = authorization_endpoint(Query(request.clone()))
            .await
            .unwrap();

        assert_eq!(response.state, request.state);
    }
}
