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
#[serde(untagged)]
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
#[serde(untagged)]
pub enum AuthorizationError {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
}

pub async fn authorization_code(
    auth_request: AuthorizationRequest,
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

    let auth_response = AuthorizationResponse {
        code: "foobar".to_string(),
        state: auth_request.state,
    };

    Ok(auth_response)
}

#[cfg(test)]
mod tests {
    use crate::authorization::{authorization_code, AuthorizationError, AuthorizationRequest, ResponseType};

    #[tokio::test]
    async fn test_authorization_code_success() {
        let request = AuthorizationRequest {
            response_type: ResponseType::Code,
            client_id: "s6BhdRkqt3".to_string(),
            state: Some("xyz".to_string()),
            redirect_uri: Some("https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb".to_string()),
            scope: None,
        };

        let response = authorization_code(request.clone()).await.unwrap();

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

        let response = authorization_code(request.clone()).await.unwrap_err();

        assert_eq!(response.state, request.state);
        assert_eq!(response.error, AuthorizationError::UnsupportedResponseType);
    }
}
