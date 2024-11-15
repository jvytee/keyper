use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AccessTokenRequest {
    pub grant_type: GrantType,
    pub code: String,
    pub redirect_uri: Option<String>,
    pub client_id: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged, rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
}

#[derive(Serialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: TokenType,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(untagged, rename_all = "snake_case")]
pub enum TokenType {
    Bearer,
}

#[derive(Serialize, Debug)]
pub struct AccessTokenErrorResponse {
    pub error: AccessTokenError,
    pub error_descripton: Option<String>,
    pub error_uri: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum AccessTokenError {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
}

pub async fn access_token(access_token_request: AccessTokenRequest) -> Result<AccessTokenResponse, AccessTokenErrorResponse> {
    if access_token_request.grant_type != GrantType::AuthorizationCode {
        let access_token_error_response = AccessTokenErrorResponse {
            error: AccessTokenError::UnsupportedGrantType,
            error_descripton: None,
            error_uri: None
        };

        return Err(access_token_error_response);
    }

    let access_token_reponse = AccessTokenResponse {
        access_token: "foobarbaz".to_string(),
        token_type: TokenType::Bearer,
        expires_in: 3600,
        refresh_token: None,
        scope: None
    };

    Ok(access_token_reponse)
}

#[cfg(test)]
mod tests {
    use crate::token::{access_token, AccessTokenRequest, GrantType, TokenType};

    #[tokio::test]
    async fn test_access_token() {
        let access_token_request = AccessTokenRequest {
            grant_type: GrantType::AuthorizationCode,
            code: "foobar".to_string(),
            redirect_uri: None,
            client_id: None,
        };

        let response = access_token(access_token_request).await.unwrap();
        assert_eq!(response.token_type, TokenType::Bearer);
    }
}
