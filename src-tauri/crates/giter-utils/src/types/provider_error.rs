use std::fmt;

pub enum ErrorCode {
  AuthorNotValidUtf8 = 0,
  ReadFileError = 1,
  GetStatusError = 2,
  IndexIsDetached = 3,    // 索引已经与HEAD分离
  BlobNotFound = 4,       // blob未找到
  NotRepoPathPrefix = 5,  // 路径不是仓库路径前缀
  RepoIsBare = 6,         // 仓库是裸仓库
  BranchNotFound = 7,      // 分支不存在
  CommitNotFound = 8,     // commit未找到
  HasConflicts = 9,       // 有冲突
  UserUnConfigured = 10,  // 用户未配置
  UnStagedFile = 11,      // 未暂存的文件
  TreeNotFound = 12,      // tree未找到
  RemoteNotFound = 13,     // remote未找到
  BranchNotTrackAny = 14,   // 分支未跟踪任何远程分支
  SshAuthorizeError = 15,     // SSH认证授权错误
  UserAuthorizeError = 16,     // 用户认证授权错误
  PushNeedNameAndPassword = 17, // 推送需要用户名和密码
  OtherError = 99999,
}
