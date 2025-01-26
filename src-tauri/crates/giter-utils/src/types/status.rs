use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// 工作状态，是否修改，是否未提交，是否未推送，是否正常
pub enum WorkStatus {
    Modified,
    Untracked,
    Uncommitted,
    Unpushed,
    Ok,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileStatus {
    Added,
    Deleted,
    Modified,
    Renamed,
    Ok,
}
