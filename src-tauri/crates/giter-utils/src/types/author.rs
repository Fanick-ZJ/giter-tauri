use serde::{Deserialize, Serialize};
use std::hash::Hash;

const DEFAULT_NAME: &str = "%%%%%%%DEFAULT_NAME%%%%%%%";
const DEFAULT_EMAIL: &str = "%%%%%%%DEFAULT_EMAIL%%%%%%%";
#[derive(Deserialize, Serialize, Hash, Debug, Clone, Default)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl Eq for Author {}

impl PartialEq for Author {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.email == other.email
    }
}

impl Author {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }
    pub fn default() -> Self {
        Self {
            name: DEFAULT_NAME.to_string(),
            email: DEFAULT_EMAIL.to_string(),
        }
    }

    pub fn is_default(&self) -> bool {
        self.name == DEFAULT_NAME && self.email == DEFAULT_EMAIL
    }
}
