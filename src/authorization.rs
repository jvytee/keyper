struct AuthorizationRequest {
    response_type: ResponseType,
    client_id: String,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>
}

enum ResponseType {
    Code,
    Token
}

struct AuthorizationResponse {
    code: String,
    state: String
}

struct AuthorizationErrorResponse {
    error: AuthorizationError,
    error_description: Option<String>,
    error_uri: Option<String>,
    state: Option<String>
}

enum AuthorizationError {
    InvalidRequest,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable
}

pub async fn handler() -> String {
    "Authorization endpoint".to_string()
}

#[cfg(test)]
mod tests {
    use super::handler;

    #[tokio::test]
    async fn test_handler() {
        let response = handler().await;
        assert_eq!(response, "Authorization endpoint");
    }
}
