use rand::{distributions, prelude::*};
use serde::{Deserialize, Serialize};

use crate::client::ClientFactory;

#[derive(Deserialize, Clone, Debug)]
pub struct AuthorizationRequest {
    pub response_type: ResponseType,
    pub client_id: String,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
    pub state: Option<String>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ResponseType {
    Code,
    Token,
}

#[derive(Serialize, Debug)]
pub struct AuthorizationResponse {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AuthorizationErrorResponse {
    pub error: AuthorizationError,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
    pub state: Option<String>,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(untagged, rename_all = "snake_case")]
pub enum AuthorizationError {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
}

pub async fn authorization_code<C: ClientFactory>(
    auth_request: AuthorizationRequest,
    client_factory: &C,
) -> Result<AuthorizationResponse, AuthorizationErrorResponse> {
    if auth_request.response_type != ResponseType::Code {
        let auth_err_response = AuthorizationErrorResponse {
            error: AuthorizationError::UnsupportedResponseType,
            error_description: None,
            error_uri: None,
            state: auth_request.state,
        };

        return Err(auth_err_response);
    }

    let Some(_client) = client_factory.from_id(&auth_request.client_id) else {
        let auth_err_response = AuthorizationErrorResponse {
            error: AuthorizationError::UnauthorizedClient,
            error_description: None,
            error_uri: None,
            state: auth_request.state,
        };

        return Err(auth_err_response);
    };

    let auth_response = AuthorizationResponse {
        code: generate_authorization_code(),
        state: auth_request.state,
    };

    Ok(auth_response)
}

fn generate_authorization_code() -> String {
    let rng = thread_rng();
    rng.sample_iter(distributions::Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        authorization::{
            authorization_code, AuthorizationError, AuthorizationRequest, ResponseType,
        },
        client::{Client, ClientFactory},
    };

    struct TestClientFactory {
        client_ids: Vec<String>,
    }

    impl ClientFactory for TestClientFactory {
        fn from_id(&self, id: &str) -> Option<crate::client::Client> {
            if self.client_ids.contains(&id.to_string()) {
                Some(Client { id: id.to_string() })
            } else {
                None
            }
        }
    }

    #[tokio::test]
    async fn test_authorization_code_success() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "s6BhdRkqt3".to_string(),
            state: Some("xyz".to_string()),
            redirect_uri: Some("https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()),
            scope: None,
        };

        let client_factory = TestClientFactory {
            client_ids: vec!["s6BhdRkqt3".to_string()],
        };

        let response = authorization_code(request.clone(), &client_factory)
            .await
            .unwrap();

        assert_eq!(response.code.len(), 24);
        assert_eq!(response.state, request.state);
    }

    #[tokio::test]
    async fn test_authorization_code_unsupported_response_type() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Token,
            client_id: "s6BhdRkqt3".to_string(),
            state: Some("xyz".to_string()),
            redirect_uri: Some("https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()),
            scope: None,
        };

        let client_factory = TestClientFactory {
            client_ids: vec!["s6BhdRkqt3".to_string()],
        };

        let response = authorization_code(request.clone(), &client_factory).await.unwrap_err();

        assert_eq!(response.state, request.state);
        assert_eq!(response.error, AuthorizationError::UnsupportedResponseType);
    }
}
