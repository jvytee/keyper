use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AuthorizationRequest {
    pub response_type: ResponseType,
    pub client_id: String,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
    pub state: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
enum ResponseType {
    Code,
    Token,
}

#[derive(Debug)]
pub struct AuthorizationResponse {
    pub code: String,
    pub state: Option<String>,
}

impl IntoResponse for AuthorizationResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

#[derive(Debug)]
pub struct AuthorizationErrorResponse {
    pub error: AuthorizationError,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
    pub state: Option<String>,
}

impl IntoResponse for AuthorizationErrorResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

#[derive(Debug)]
enum AuthorizationError {
    InvalidRequest,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
}

pub async fn handler(
    Query(auth_request): Query<AuthorizationRequest>,
) -> Result<AuthorizationResponse, AuthorizationErrorResponse> {
    let auth_response = AuthorizationResponse {
        code: "foobar".to_string(),
        state: auth_request.state,
    };

    Ok(auth_response)
}

#[cfg(test)]
mod tests {
    use axum::extract::Query;

    use crate::authorization::{handler, AuthorizationRequest, ResponseType};

    #[tokio::test]
    async fn test_handler() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "s6BhdRkqt3".to_string(),
            state: Some("xyz".to_string()),
            redirect_uri: Some("https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()),
            scope: None,
        };
        let response = handler(Query(request.clone())).await.unwrap();
        assert_eq!(response.state, request.state);
    }
}
