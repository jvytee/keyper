use std::sync::Arc;

use axum::{extract::State, response::Html};
use tera::Context;

use super::RouterState;

pub async fn authentication_endpoint(State(router_state): State<Arc<RouterState>>) -> Html<String> {
    let context = Context::new();
    let html = router_state
        .template_engine
        .render("authenticate", &context)
        .expect("TODO: Handle this properly");

    Html(html)
}

#[cfg(test)]
mod tests {
    use axum::{extract::State, response::Html};
    use core::{assert_eq, assert_ne};
    use std::sync::Arc;

    use crate::{
        api::{RouterState, authentication::authentication_endpoint, create_template_engine},
        data::client::TestClientStore,
    };

    #[tokio::test]
    async fn test_authentication_endpoint() {
        let client_store = TestClientStore {
            client_ids: vec!["foobar".to_string()],
        };
        let template_engine = create_template_engine().expect("Could not create template engine");
        let router_state = RouterState {
            client_store,
            template_engine,
        };

        let Html(response) = authentication_endpoint(State(Arc::new(router_state))).await;
        assert_ne!(response.len(), 0);
    }
}
