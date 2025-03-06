use crate::make_serializable;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandError {
    pub message: String,
    pub code: CommandErrorEnum,
    // data 需要容纳函数的传入参数
    pub data: Option<Vec<String>>,
}


make_serializable! {
    #[derive(Deserialize, Debug, Clone)]
    pub enum CommandErrorEnum {
        RepositoryHasWatched,   // 仓库已经被监听
        AddWatcherError,        // 添加监听失败
        AddRepositoryStoreError,// 添加仓库失败
        InvalidRepository,      // 无效的仓库
        RepoHasnotOwnership,    // 仓库没有所有权
        FindAuthorsError,       // 查找作者失败
        DataProviderBuildError, // 构建数据提供者失败
        BranchNotFound,         // 分支不存在
        BranchesFindError,      // 查找分支失败
        GetAuthorError,         // 获取作者失败
        DbNotFound,             // 数据库不存在
        GetFoldersError,        // 获取文件夹失败
        GetWorkStatusError,     // 获取工作状态失败
        SetRepoOwnershipError,  // 设置仓库所有权失败
        RemoveWatcherError,     // 移除监听失败
        GetBranchCommitsError,  // 获取提交失败
        GetCurrentBranchError,  // 获取当前分支失败
        ConvertOidError,        // 转换OID失败
        GetCommitContentError,  // 获取提交内容失败
        GetFileDiffError,       // 获取文件差异失败
        GetFileContentError,    // 获取文件内容失败
        GetCommitError,         // 获取提交失败
        GetBranchCommitContributionError, // 获取分支提交贡献失败
        GetGlobalAuthorError,   // 获取全局作者失败
        GetRepoAuthorError,     // 获取仓库作者失败
        GetChangedFilesError,   // 获取变更文件失败
        GetStagedFilesError,    // 获取暂存文件失败
        AddToStageError,        // 添加到暂存区失败
        RemoveFromStageError,   // 从暂存区移除失败
        CheckoutFileError,      // 检出文件失败
        CommitError,            // 提交失败
        GetCurrentRemoteBranchError, // 获取当前远程分支失败
        PushError,              // 推送失败
    }
}
