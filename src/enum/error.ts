
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
	CommitNotFound: 9,
	CurrentBranchNotFound: 10,
	HasConflicts: 11,
	UserUnConfigured: 12,
	UnStagedFile: 13,
	TreeNotFound: 14,
	RemoteNotFound: 15,
	BranchNotTrackAny: 16,
	SshAuthorizeError: 17,
	UserAuthorizeError: 18,
	RemoteHeadHasNotInLocal: 19,
	PushNeedNameAndPassword: 20,
	RepoAuthorNoConfig: 21,
	RepoHasConflicts: 22,
	NoStagedFile: 23,
	PushOtherError: 24,
	InvalidFilePath: 25,
	TargetReferenceNotDirect: 26,
	BuildMergeCommitError: 27,
	CommitBeforePullWouldBeOverwrittenByMerge: 28,
	CantPull: 29,
	OtherError: 30,
	Git2Error: 31,
	AnyhowError: 32
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
