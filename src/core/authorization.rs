use rand::{distributions, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct AuthorizationRequest {
    pub response_type: ResponseType,
    pub client_id: String,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
    pub state: Option<String>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
pub enum AuthorizationError {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
}

#[derive(Debug)]
pub struct AuthorizationSuccessResponse(pub AuthorizationResponse, pub String);

pub trait ClientStore {
    fn read_client(&self, id: &str) -> Option<Client>;
}

#[derive(Deserialize, Debug)]
pub struct Client {
    pub id: String,
    pub client_type: ClientType,
    pub redirect_uris: Vec<String>,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClientType {
    Confidential,
    Public,
}

pub async fn authorization_code<C: ClientStore>(
    auth_request: AuthorizationRequest,
    client_store: &C,
) -> Result<AuthorizationSuccessResponse, AuthorizationErrorResponse> {
    if auth_request.response_type != ResponseType::Code {
        return Err(AuthorizationErrorResponse {
            error: AuthorizationError::UnsupportedResponseType,
            error_description: None,
            error_uri: None,
            state: auth_request.state,
        });
    }

    let Some(client) = client_store.read_client(&auth_request.client_id) else {
        return Err(AuthorizationErrorResponse {
            error: AuthorizationError::UnauthorizedClient,
            error_description: None,
            error_uri: None,
            state: auth_request.state,
        });
    };

    let redirect_uri = match (auth_request.redirect_uri, &client.redirect_uris.as_slice()) {
        (None, &[]) => Err(AuthorizationError::InvalidRequest),
        (None, &[redirect_uri, ..]) => Ok(redirect_uri.to_string()),
        (Some(redirect_uri), &[]) => Ok(redirect_uri),
        (Some(redirect_uri), redirect_uris) => {
            if redirect_uris.contains(&redirect_uri) {
                Ok(redirect_uri)
            } else {
                Err(AuthorizationError::InvalidRequest)
            }
        }
    };

    match redirect_uri {
        Err(error) => Err(AuthorizationErrorResponse {
            error,
            error_description: None,
            error_uri: None,
            state: auth_request.state,
        }),
        Ok(redirect_uri) => Ok(AuthorizationSuccessResponse(
            AuthorizationResponse {
                code: generate_authorization_code(),
                state: auth_request.state,
            },
            redirect_uri,
        )),
    }
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
    use crate::core::authorization::{
        authorization_code, AuthorizationError, AuthorizationRequest, AuthorizationSuccessResponse,
        Client, ClientStore, ClientType, ResponseType,
    };

    use super::generate_authorization_code;

    struct TestClientStore {
        client_ids: Vec<String>,
        redirect_uris: Vec<Vec<String>>,
    }

    impl ClientStore for TestClientStore {
        fn read_client(&self, id: &str) -> Option<Client> {
            self.client_ids
                .iter()
                .position(|elem| elem == id)
                .map(|index| Client {
                    id: self.client_ids[index].clone(),
                    client_type: ClientType::Public,
                    redirect_uris: self.redirect_uris[index].clone(),
                    name: "Example Client".to_string(),
                })
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

        let client_store = TestClientStore {
            client_ids: vec!["s6BhdRkqt3".to_string()],
            redirect_uris: vec![vec!["https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()]],
        };

        let response = authorization_code(request.clone(), &client_store).await;

        assert!(response.is_ok());
        let AuthorizationSuccessResponse(response, redirect_uri) = response.unwrap();

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

        let client_store = TestClientStore {
            client_ids: vec!["s6BhdRkqt3".to_string()],
            redirect_uris: vec![vec!["https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()]],
        };

        let response = authorization_code(request.clone(), &client_store).await;

        assert!(response.is_err());
        let response = response.unwrap_err();

        assert_eq!(response.error, AuthorizationError::UnsupportedResponseType);
        assert_eq!(response.state, request.state);
    }

    #[tokio::test]
    async fn test_authorization_code_unauthorized_client() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "wrong".to_string(),
            redirect_uri: Some("https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()),
            scope: None,
            state: Some("xyz".to_string()),
        };

        let client_store = TestClientStore {
            client_ids: vec!["s6BhdRkqt3".to_string()],
            redirect_uris: vec![vec!["https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()]],
        };

        let response = authorization_code(request.clone(), &client_store).await;

        assert!(response.is_err());
        let response = response.unwrap_err();

        assert_eq!(response.error, AuthorizationError::UnauthorizedClient);
        assert_eq!(response.state, request.state);
    }

    #[tokio::test]
    async fn test_authorization_code_no_redirect_url() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "s6BhdRkqt3".to_string(),
            redirect_uri: None,
            scope: None,
            state: Some("xyz".to_string()),
        };

        let client_store = TestClientStore {
            client_ids: vec!["s6BhdRkqt3".to_string()],
            redirect_uris: vec![Vec::new()],
        };

        let response = authorization_code(request.clone(), &client_store).await;

        assert!(response.is_err());
        let response = response.unwrap_err();

        assert_eq!(response.error, AuthorizationError::InvalidRequest);
        assert_eq!(response.state, request.state);
    }

    #[tokio::test]
    async fn test_authorization_code_hard_redirect_url() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "s6BhdRkqt3".to_string(),
            redirect_uri: None,
            scope: None,
            state: Some("xyz".to_string()),
        };

        let client_store = TestClientStore {
            client_ids: vec!["s6BhdRkqt3".to_string()],
            redirect_uris: vec![vec!["https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()]],
        };

        let response = authorization_code(request.clone(), &client_store).await;

        assert!(response.is_ok());
        let AuthorizationSuccessResponse(response, redirect_uri) = response.unwrap();

        assert_eq!(
            redirect_uri,
            "https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()
        );
        assert_eq!(response.state, request.state);
    }

    #[tokio::test]
    async fn test_authorization_code_dyn_redirect_url() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "s6BhdRkqt3".to_string(),
            redirect_uri: Some("https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()),
            scope: None,
            state: Some("xyz".to_string()),
        };

        let client_store = TestClientStore {
            client_ids: vec!["s6BhdRkqt3".to_string()],
            redirect_uris: vec![vec!["https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()]],
        };

        let response = authorization_code(request.clone(), &client_store).await;

        assert!(response.is_ok());
        let AuthorizationSuccessResponse(response, redirect_uri) = response.unwrap();

        assert_eq!(
            redirect_uri,
            "https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()
        );
        assert_eq!(response.state, request.state);
    }

    #[tokio::test]
    async fn test_authorization_code_invalid_redirect_url() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "s6BhdRkqt3".to_string(),
            redirect_uri: Some("https%3A%2F%2Fclient%2Eexample%2Ecom".to_string()),
            scope: None,
            state: Some("xyz".to_string()),
        };

        let client_store = TestClientStore {
            client_ids: vec!["s6BhdRkqt3".to_string()],
            redirect_uris: vec![vec!["https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()]],
        };

        let response = authorization_code(request.clone(), &client_store).await;

        assert!(response.is_err());
        let response = response.unwrap_err();

        assert_eq!(response.error, AuthorizationError::InvalidRequest);
        assert_eq!(response.state, request.state);
    }

    #[test]
    fn test_generate_authorization_code() {
        let auth_code = generate_authorization_code();
        assert_eq!(auth_code.len(), 24);
    }
}
