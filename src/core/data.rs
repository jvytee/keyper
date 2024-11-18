pub trait ClientFactory {
    fn from_id(&self, id: &str) -> Option<Client>;
}

pub struct Client {
    pub id: String,
    pub scopes: Vec<String>
}

pub trait UserFactory {
    fn from_name(&self, name: &str) -> Option<User>;
}

pub struct User {
    pub name: String,
    pub hash: String
}
