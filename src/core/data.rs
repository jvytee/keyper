use std::error::Error;

use chrono::Utc;
use serde::Deserialize;

pub trait ClientStore {
    fn read_client(&self, id: &str) -> Option<Client>;
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

pub trait OwnerStore {
    fn read_owner(&self, name: &str) -> Option<Owner>;
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub email: String,
    pub name: String,
    pub hash: String,
}

pub trait AuthorizationStore {
    fn create_authorization<E: Error>(&self, authorization: &Authorization) -> Result<(), E>;
    fn read_authorization(&self, token: &str) -> Option<Authorization>;
}

pub struct Authorization {
    access_token: String,
    scopes: Vec<String>,
    owner_id: i32,
    created: chrono::DateTime<Utc>,
    expires: chrono::DateTime<Utc>,
    refresh_token: Option<String>,
}
