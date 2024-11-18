use crate::core::data::{Client, ClientFactory};

#[derive(Clone, Debug)]
pub struct TestClientFactory {
    pub client_ids: Vec<String>,
}

impl ClientFactory for TestClientFactory {
    fn from_id(&self, id: &str) -> Option<Client> {
        if self.client_ids.contains(&id.to_string()) {
            Some(Client { id: id.to_string() })
        } else {
            None
        }
    }
}
