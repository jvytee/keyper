use crate::core::data::{Client, ClientSource};

#[derive(Clone, Debug)]
pub struct TestClientSource {
    pub client_ids: Vec<String>,
}

impl ClientSource for TestClientSource {
    fn by_id(&self, id: &str) -> Option<Client> {
        if self.client_ids.contains(&id.to_string()) {
            Some(Client {
                id: id.to_string(),
                name: "Example Client".to_string(),
                scopes: vec!["foo".to_string(), "bar".to_string()],
            })
        } else {
            None
        }
    }
}
