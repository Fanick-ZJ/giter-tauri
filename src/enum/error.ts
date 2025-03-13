export enum CommandError {
  RepositoryHasWatched = 'REPOSITORY_HAS_WATCHED',
  AddWatcherError = 'AddWatcherError',
  AddRepositoryStoreError = 'AddRepositoryStoreError',
  InvalidRepository = 'InvalidRepository',
  RepoHasnotOwnership = 'RepoHasnotOwnership',
  FindAuthorsError = 'FindAuthorsError',
  DataProviderBuildError = 'DataProviderBuildError',
  BranchNotFound = 'BranchNotFound',
  BranchesFindError = 'BranchesFindError',
  GetAuthorError = 'GetAuthorError',
  DbNotFound = 'DbNotFound',
  GetFoldersError = 'GetFoldersError',
  GetWorkStatusError = 'GetWorkStatusError',
  SetRepoOwnershipError = 'SetRepoOwnershipError',
}

export enum ReasonErrorCode {
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
  BranchNotFind = 14,      // 分支未找到
  BranchNotTrackAny = 15,   // 分支未跟踪任何远程分支
  SshAuthorizeError = 16,     // SSH认证授权错误
  UserAuthorizeError = 17,     // 用户认证授权错误
  RemoteHeadHasNotInLocal = 18,// 远程HEAD未在本地
  PushNeedNameAndPassword = 19, // 推送需要用户名和密码
  PushOtherError = 20,     // 推送其他错误
  OtherError = 99999,
}
