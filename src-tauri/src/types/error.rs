use crate::make_serializable;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

make_serializable! {
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
        SetRepoOwnershipError(String),  // 设置仓库所有权失败
        RemoveWatcherError(String),     // 移除监听失败
        GetBranchCommitsError(String),  // 获取提交失败
        GetCurrentBranchError(String),  // 获取当前分支失败
        ConvertOidError(String),        // 转换OID失败
        GetCommitContentError(String),  // 获取提交内容失败
        GetFileDiffError(String),       // 获取文件差异失败
    }
}
