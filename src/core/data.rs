use std::collections::HashMap;
use serde::Deserialize;

pub trait ClientSource {
    fn by_id(&self, id: &str) -> Option<Client>;
}

#[derive(Deserialize, Debug)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub scopes: Vec<String>
}

pub trait UserSource {
    fn by_name(&self, name: &str) -> Option<User>;
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hash: String,
    pub client_scopes: HashMap<String, Vec<String>>
}
