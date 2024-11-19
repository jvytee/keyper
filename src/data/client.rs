use crate::core::data::{Client, ClientSource, ClientType};

#[derive(Clone, Debug)]
pub struct TestClientSource {
    pub client_ids: Vec<String>,
}

impl ClientSource for TestClientSource {
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
