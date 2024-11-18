use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::authorization::{
    self, AuthorizationErrorResponse, AuthorizationRequest, AuthorizationResponse,
};

use super::RouterState;

pub async fn authorization_endpoint(
    State(router_state): State<Arc<RouterState>>,
    Query(auth_request): Query<AuthorizationRequest>,
    headers: HeaderMap,
) -> Response {
    if !headers.contains_key("Authorization") {
        return (
            StatusCode::UNAUTHORIZED,
            [(header::WWW_AUTHENTICATE, "Basic realm=\"Authorization\"")],
            (),
        )
            .into_response();
    }

    authorization::authorization_code(auth_request, &router_state.client_factory)
        .await
        .into_response()
}

impl IntoResponse for AuthorizationResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for AuthorizationErrorResponse {
    fn into_response(self) -> Response {
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
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        extract::{Query, State},
        http::{HeaderMap, StatusCode},
    };

    use crate::{
        api::{authorization::authorization_endpoint, RouterState},
        authorization::{AuthorizationRequest, ResponseType},
        client::TestClientFactory,
    };

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

        let mut headers = HeaderMap::new();
        // headers.insert("Authorization", "foobarbaz".parse().unwrap());

        let response = authorization_endpoint(
            State(Arc::new(router_state)),
            Query(request.clone()),
            headers,
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
