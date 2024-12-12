use std::collections::HashMap;

use serde::Deserialize;

use crate::core::authorization::{Client, ClientStore, ClientType};

#[derive(Clone, Debug)]
pub struct MapClientStore {
    pub data: HashMap<String, ClientData>,
}

impl ClientStore for MapClientStore {
    fn read_client(&self, id: &str) -> Option<Client> {
        self.data.get(id).map(|client_data| Client {
            id: id.to_string(),
            client_type: client_data.client_type.clone(),
            redirect_uris: client_data.redirect_uris.clone(),
            name: client_data.name.clone(),
        })
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ClientData {
    pub client_type: ClientType,
    pub redirect_uris: Vec<String>,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct TestClientStore {
    pub client_ids: Vec<String>,
}

impl ClientStore for TestClientStore {
    fn read_client(&self, id: &str) -> Option<Client> {
        if self.client_ids.contains(&id.to_string()) {
            Some(Client {
                id: id.to_string(),
                client_type: ClientType::Public,
                redirect_uris: Vec::new(),
                name: "Example Client".to_string(),
            })
        } else {
            None
        }
    }
}
