use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileStatus {
    modified,
    untracked,
    uncommited,
    unpushed,
    ok
}