#[derive(Debug, Clone)]
pub enum ErrorCode {
  AuthorNotValidUtf8,
  ReadFileError ,
  GetStatusError ,
  IndexIsDetached ,    // 索引已经与HEAD分离
  BlobNotFound ,       // blob未找到
  NotRepoPathPrefix ,  // 路径不是仓库路径前缀
  RepoIsBare,         // 仓库是裸仓库
  BranchNotFound ,      // 分支不存在
  CommitNotFound ,     // commit未找到
  HasConflicts ,       // 有冲突
  UserUnConfigured ,  // 用户未配置
  UnStagedFile ,      // 未暂存的文件
  TreeNotFound ,      // tree未找到
  RemoteNotFound ,     // remote未找到
  BranchNameInvalid,  // 分支名称无效
  BranchNotFind ,      // 分支未找到
  BranchNotTrackAny ,   // 分支未跟踪任何远程分支
  SshAuthorizeError ,     // SSH认证授权错误
  UserAuthorizeError ,     // 用户认证授权错误
  RemoteHeadHasNotInLocal ,// 远程HEAD未在本地
  PushNeedNameAndPassword , // 推送需要用户名和密码
  RepoAuthorNoConfig, // 仓库作者未配置
  RepoHasConflicts, // 仓库有冲突
  NoStagedFile,     // 没有暂存的文件
  PushOtherError ,     // 推送其他错误
  InvalidFilePaht, // 无效的文件路径
  NotUtf8Path, // 不是UTF-8路径
  OtherError ,
}


impl From<ErrorCode> for anyhow::Error {
  fn from(err: ErrorCode) -> Self {
    anyhow::anyhow!(err as i32)
  } 
}
impl From<ErrorCode> for i32 {
  fn from(err: ErrorCode) -> Self {
    err as i32
  } 
}