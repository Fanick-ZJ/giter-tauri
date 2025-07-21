use serde::{Deserialize, Serialize};

///就像是 refs/remotes/origin/HEAD 这样完整的名字
///
type Reference = String;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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

impl<'a> From<git2::Reference<'a>> for Branch {
    fn from(reference: git2::Reference) -> Self {
        let name: String = reference.shorthand().unwrap_or_default().to_string();
        let reference_str = reference.name().unwrap_or_default().to_string();

        Branch {
            name,
            is_remote: reference_str.starts_with("refs/remotes/"),
            reference: reference_str,
        }
    }
}
