pub trait UserFactory {
    fn from_name(&self, name: &str) -> Option<User>;
}

pub struct User {
    pub name: String,
    pub hash: String
}
