use std::string::FromUtf8Error;

use giter_traits::ExposeError;
use strum_macros::{EnumDiscriminants, EnumIter};

use thiserror::Error;

#[derive(Error, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum GitUtilsErrorCode {
    #[error("not valid utf8: {0}")]
    NotValidUtf8S(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("not valid utf8: {0}")]
    NotValidUtf8F(#[from] FromUtf8Error),

    #[error("read file error: {0}")]
    ReadFileError(String),

    #[error("This repository have not owner: {0}")]
    NoOwner(String),

    #[error("failed to get repository status: {0}")]
    GetStatusError(String),

    #[error("The index head is detached: {0}")]
    IndexIsDetached(String), // 索引已经与HEAD分离

    #[error("The repository:{0} not found")]
    RepoNotFound(String), // 仓库不存在

    #[error("blob not found: {0}")]
    BlobNotFound(String),

    #[error("This repository is bare: {0}")]
    RepoIsBare(String), // 仓库是裸仓库

    #[error("Branch not found: {0}")] // 分支不存在
    BranchNotFound(String), // 分支不存在

    #[error("Failed to switch branch: {0}")] // 切换分支失败
    SwitchBranchError(String),

    #[error("Commit not found: {0}")] // Commit不存在
    CommitNotFound(String),

    #[error("Current branch found error: {0}")] // 当前分支获取失败
    CurrentBranchNotFound(String), // 当前分支获取失败

    #[error("has conflicts")]
    HasConflicts, // 有冲突

    #[error("Not found user configuration in local")]
    UserUnConfigured, // 用户未配置

    #[error("Not found the staged file")]
    UnStagedFile, // 未暂存的文件

    #[error("The tree not found:{0}")]
    TreeNotFound(String), // tree未找到

    #[error("The remote not found:{0}")]
    RemoteNotFound(String), // remote未找到

    #[error("The branch {0} not track any branch")]
    BranchNotTrackAny(String), // 分支未跟踪任何远程分支

    #[error("Ssh authorize error")]
    SshAuthorizeError, // SSH认证授权错误

    #[error("User authorize error")]
    UserAuthorizeError, // 用户认证授权错误

    #[error("Remote HEAD not in local")]
    RemoteHeadHasNotInLocal, // 远程HEAD未在本地

    #[error("Push need name and password")]
    PushNeedNameAndPassword, // 推送需要用户名和密码

    #[error("Repository author not configured")]
    RepoAuthorNoConfig, // 仓库作者未配置

    #[error("Repository has conflicts: {0}")]
    RepoHasConflicts(String), // 仓库有冲突

    #[error("No staged file")]
    NoStagedFile, // 没有暂存的文件

    #[error("No changes")]
    PushOtherError, // 推送其他错误

    #[error("Invalid file path: {0}")]
    InvalidFilePath(String), // 无效的文件路径

    #[error("Target reference is not direct")]
    TargetReferenceNotDirect, // 目标引用不是直接引用

    #[error("Switch will be overwritten by merge: {0}")]
    SwitchWillBeOverwrittenByMerge(String), // 切换将被合并覆盖

    #[error("Build merge commit error")]
    BuildMergeCommitError, // 构建合并提交错误

    #[error("Commit before pull would be overwritten by merge")]
    CommitBeforePullWouldBeOverwrittenByMerge, // 提交将被合并覆盖

    #[error("Cant pull")]
    CantPull, // 不能拉取

    #[error("Other git error: {0}")]
    OtherError(String),

    #[error("data store disconnected")]
    Git2Error(#[from] git2::Error),

    #[error("other error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}

impl ExposeError for GitUtilsErrorCode {
    fn code(&self) -> u32 {
        GitUtilsErrorCodeDiscriminants::from(self) as u32
    }

    fn module(&self) -> &str {
        return "giter-utils";
    }
}
