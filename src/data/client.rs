use crate::core::data::{Client, ClientStore, ClientType};

#[derive(Clone, Debug)]
pub struct TestClientStore {
    pub client_ids: Vec<String>,
}

impl ClientStore for TestClientStore {
    fn get_client(&self, id: &str) -> Option<Client> {
        if self.client_ids.contains(&id.to_string()) {
            Some(Client {
                id: id.to_string(),
                client_type: ClientType::Public,
                redirection_uris: Vec::new(),
                name: "Example Client".to_string(),
            })
        } else {
            None
        }
    }
}