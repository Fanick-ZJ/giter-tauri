use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: i32,
    pub path: String,
    pub alias: String,
    pub has_watch: bool,
    pub order: i32,
    pub top: bool,
}

impl Repository {
    pub fn new(
        id: i32,
        path: String,
        alias: String,
        has_watch: bool,
        order: i32,
        top: bool,
    ) -> Repository {
        Repository {
            id,
            path,
            alias,
            has_watch,
            order,
            top,
        }
    }
}
