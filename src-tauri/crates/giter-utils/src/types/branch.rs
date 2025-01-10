use serde::{Deserialize, Serialize};

///就像是 refs/remotes/origin/HEAD 这样完整的名字
///
type Reference = String;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch {
    pub name: String,
    pub is_remote: bool,
    pub reference: Reference,
}

impl Branch {
    pub fn new(name: String, is_remote: bool, reference: String) -> Self {
        Branch {
            name,
            is_remote,
            reference,
        }
    }
}
