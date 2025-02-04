use serde::{Deserialize, Serialize};
use similar::DiffOp;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContentDiff {
    pub old: String,
    pub new: String,
    pub ops: Vec<DiffOp>,
    pub display: String,
}