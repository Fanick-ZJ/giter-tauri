use bitflags::bitflags;
use git2::Status;
use serde::{Deserialize, Serialize};

bitflags! {
  /// 工作状态，是否修改，是否未提交，是否未推送，是否正常
  /// Modified状态中包含工作空间的新建、修改和删除
  #[derive(Debug, Clone)]
  pub struct WorkStatus: u32 {
    const None = 0;
    const Ok = 1;
    const Added = 1 << 1;
    const Modified = 1 << 2;
    const Untracked = 1 << 3;
    const Uncommitted = 1 << 4;
    const Unpushed =  1 << 5;
  }
}

impl Serialize for WorkStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FileStatus {
    Added,
    Deleted,
    Modified,
    Renamed,
    Conflicted,
    Ok,
}


  /// git2中的状态转化为文件状态（是否添加、修改删除）
pub fn status_to_changed_status(status: Status) -> FileStatus {
    match status {
        Status::WT_NEW | Status::INDEX_NEW => FileStatus::Added,
        Status::WT_MODIFIED | Status::INDEX_MODIFIED => FileStatus::Modified,
        Status::WT_DELETED | Status::INDEX_DELETED => FileStatus::Deleted, 
        Status::WT_RENAMED | Status::INDEX_RENAMED => FileStatus::Renamed,
        Status::CONFLICTED => FileStatus::Conflicted,
        _ => FileStatus::Ok,
    }
}