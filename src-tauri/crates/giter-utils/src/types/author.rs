use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Deserialize, Serialize, Hash, Debug, PartialEq, Clone)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl Eq for Author {}

impl Author {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }
}
