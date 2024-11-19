use std::error::Error;

use chrono::Utc;
use serde::Deserialize;

pub trait ClientSource {
    fn get_client(&self, id: &str) -> Option<Client>;
}

#[derive(Deserialize, Debug)]
pub struct Client {
    pub id: String,
    pub client_type: ClientType,
    pub redirection_uris: Vec<String>,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClientType {
    Confidential,
    Public,
}

pub trait OwnerSource {
    fn get_owner(&self, name: &str) -> Option<Owner>;
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub id: i32,
    pub name: String,
    pub hash: String,
}

pub trait TokenStore {
    fn get_token(&self, value: &str) -> Option<Token>;
    fn set_token<E: Error>(&self, token: &Token) -> Result<(), E>;
}

pub struct Token {
    value: String,
    created: chrono::DateTime<Utc>,
    expires: chrono::DateTime<Utc>,
}
