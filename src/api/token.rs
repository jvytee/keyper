use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::core::token::{access_token, AccessTokenErrorResponse, AccessTokenRequest, AccessTokenResponse};
use super::RouterState;

pub async fn token_endpoint(
    State(router_state): State<Arc<RouterState>>,
    Query(access_token_request): Query<AccessTokenRequest>,
) -> Result<AccessTokenResponse, AccessTokenErrorResponse> {
    access_token(access_token_request).await
}

impl IntoResponse for AccessTokenResponse {
    fn into_response(self) -> Response {
        todo!()
    }
}

impl IntoResponse for AccessTokenErrorResponse {
    fn into_response(self) -> Response {
        todo!()
    }
}
