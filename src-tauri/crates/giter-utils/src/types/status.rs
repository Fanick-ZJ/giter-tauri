use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WorkStatus {
    Modified,
    Untracked,
    Uncommited,
    Unpushed,
    Ok
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileStatus {
    Added,
    Deleted,
    Modified,
    Renamed
}