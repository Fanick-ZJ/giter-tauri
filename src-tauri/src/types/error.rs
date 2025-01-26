use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Debug)]
pub enum CommandError {
    RepositoryHasWatched(String),   // 仓库已经被监听
    AddWatcherError(String),        // 添加监听失败
    AddRepositoryStoreError(String),// 添加仓库失败
    InvalidRepository(String),      // 无效的仓库
    RepoHasnotOwnership(String),    // 仓库没有所有权
    FindAuthorsError(String),       // 查找作者失败
    DataProviderBuildError(String), // 构建数据提供者失败
    BranchNotFound(String),         // 分支不存在
    BranchesFindError(String),      // 查找分支失败
    GetAuthorError(String),         // 获取作者失败
    DbNotFound(String),             // 数据库不存在
    GetFoldersError(String),        // 获取文件夹失败
    GetWorkStatusError(String),     // 获取工作状态失败
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CommandError", 2)?;
        match *self {
            CommandError::RepositoryHasWatched(ref path) => {
                s.serialize_field("error", "RepositoryHasWatched")?;
                s.serialize_field("data", path)?;
            },
            CommandError::AddWatcherError(ref path) => {
                s.serialize_field("error", "AddWatcherError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::AddRepositoryStoreError(ref path) => {
                s.serialize_field("error", "AddRepositoryStoreError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::InvalidRepository(ref path) => {
                s.serialize_field("error", "InvalidRepository")?;
                s.serialize_field("data", path)?;
            },
            CommandError::FindAuthorsError(ref path) => {
                s.serialize_field("error", "FindAuthorsError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::BranchNotFound(ref path) => {
                s.serialize_field("error", "BranchNotFound")?;
                s.serialize_field("data", path)?;
            },
            CommandError::DataProviderBuildError(ref path) => {
                s.serialize_field("error", "DataProviderBuildError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::GetAuthorError(ref path) => {
                s.serialize_field("error", "GetAuthorError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::BranchesFindError(ref path) => {
                s.serialize_field("error", "BranchesFindError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::DbNotFound(ref path) => {
                s.serialize_field("error", "DbNotFound")?;
                s.serialize_field("data", path)?;
            },
            CommandError::GetFoldersError(ref path) => {
                s.serialize_field("error", "GetFoldersError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::GetWorkStatusError(ref path) => {
                s.serialize_field("error", "GetWorkStatusError")?;
                s.serialize_field("data", path)?; 
            },
            CommandError::RepoHasnotOwnership(ref path) => {
                s.serialize_field("error", "RepoHasnotOwnership")?;
                s.serialize_field("data", path)?; 
            }
        }
        s.end()
    }
}
