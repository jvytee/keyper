use std::collections::HashMap;

use serde::Deserialize;

use crate::core::authorization::{Client, ClientStore, ClientType};

#[derive(Clone, Debug)]
pub struct MapClientStore {
    pub data: HashMap<String, ClientData>,
}

impl MapClientStore {
    pub fn try_from_toml(input: &str) -> Result<Self, toml::de::Error> {
        let data: HashMap<String, ClientData> = toml::from_str(input)?;
        Ok(Self { data })
    }
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

#[cfg(test)]
mod test {
    use crate::core::authorization::{ClientStore, ClientType};

    use super::MapClientStore;

    #[test]
    fn test_try_from_toml() {
        let input = r#"
            [abcd1234]
            name = "TestClient"
            client_type = "public"
            redirect_uris = ["https://example.com/auth_success"]
        "#;

        let client_store = MapClientStore::try_from_toml(input).unwrap();
        assert_eq!(client_store.data.len(), 1);

        let test_client = client_store.read_client("abcd1234").unwrap();
        assert_eq!(test_client.id, "abcd1234");
        assert_eq!(test_client.name, "TestClient");
        assert_eq!(test_client.client_type, ClientType::Public);
        assert_eq!(test_client.redirect_uris.len(), 1);
        assert_eq!(
            test_client.redirect_uris[0],
            "https://example.com/auth_success"
        );
    }
}
