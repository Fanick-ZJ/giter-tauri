
export const GitUtilsErrorCode = {
  NotValidUtf8: 0,
	ReadFileError: 1,
	NoOwner: 2,
	GetStatusError: 3,
	IndexIsDetached: 4,
	RepoNotFound: 5,
	BlobNotFound: 6,
	RepoIsBare: 7,
	BranchNotFound: 8,
	SwitchBranchError: 9,
	CommitNotFound: 10,
	CurrentBranchNotFound: 11,
	HasConflicts: 12,
	UserUnConfigured: 13,
	UnStagedFile: 14,
	TreeNotFound: 15,
	RemoteNotFound: 16,
	BranchNotTrackAny: 17,
	SshAuthorizeError: 18,
	UserAuthorizeError: 19,
	RemoteHeadHasNotInLocal: 20,
	PushNeedNameAndPassword: 21,
	RepoAuthorNoConfig: 22,
	RepoHasConflicts: 23,
	NoStagedFile: 24,
	PushOtherError: 25,
	InvalidFilePath: 26,
	TargetReferenceNotDirect: 27,
	BuildMergeCommitError: 28,
	CommitBeforePullWouldBeOverwrittenByMerge: 29,
	CantPull: 30,
	OtherError: 31,
	Git2Error: 32,
	AnyhowError: 33
} as const

export const WatcherErrorCode = {
  AddWatcherFailed: 0,
	RemoveWatcherFailed: 1,
	Other: 2
} as const

export const CommonErrorCode = {
  GetWatcherCenterFailed: 0,
	GetReposFailed: 1,
	DatabaseInvalid: 2,
	PathInvalid: 3,
	SetGlobalConfigError: 4,
	GetGlobalConfigError: 5
} as const
