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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClientType {
    Confidential,
    Public,
}

pub async fn authorization_code<C: ClientStore>(
    auth_request: AuthorizationRequest,
    client_store: &C,
) -> Result<AuthorizationResponse, AuthorizationErrorResponse> {
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

    if let Err(error) = redirect_uri {
        return Err(AuthorizationErrorResponse {
            error,
            error_description: None,
            error_uri: None,
            state: auth_request.state,
        });
    }

    Ok(AuthorizationResponse {
        code: generate_authorization_code(),
        state: auth_request.state,
    })
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
            authorization_code, AuthorizationError, AuthorizationRequest, Client, ClientStore, ClientType, ResponseType
        };

    struct TestClientStore {
        client_ids: Vec<String>,
    }

    impl ClientStore for TestClientStore {
        fn read_client(&self, id: &str) -> Option<Client> {
            if self.client_ids.contains(&id.to_string()) {
                Some(Client {
                    id: id.to_string(),
                    client_type: ClientType::Public,
                    redirect_uris: Vec::new(),
                    name: "Example Client".to_string(),
                })
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

        let client_store = TestClientStore {
            client_ids: vec!["s6BhdRkqt3".to_string()],
        };

        let response = authorization_code(request.clone(), &client_store)
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

        let client_store = TestClientStore {
            client_ids: vec!["s6BhdRkqt3".to_string()],
        };

        let response = authorization_code(request.clone(), &client_store)
            .await
            .unwrap_err();

        assert_eq!(response.state, request.state);
        assert_eq!(response.error, AuthorizationError::UnsupportedResponseType);
    }
}
