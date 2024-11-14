struct AccessTokenRequest {
    grant_type: GrantType,
    code: String,
    redirect_uri: Option<String>,
    client_id: Option<String>
}

enum GrantType {
    AuthorizationCode
}

struct AccessTokenResponse {
    access_token: String,
    token_type: TokenType,
    expires_in: i64,
    refresh_token: Option<String>,
    scope: Option<String>
}

enum TokenType {
    Bearer
}

struct AccessTokenErrorResponse {
    error: AccessTokenError,
    error_descripton: Option<String>,
    error_uri: Option<String>
}

enum AccessTokenError {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope
}

pub async fn handler() -> String {
    "Token endpoint".to_string()
}

#[cfg(test)]
mod tests {
    use super::handler;

    #[tokio::test]
    async fn test_handler() {
        let response = handler().await;
        assert_eq!(response, "Token endpoint");
    }
}
