
export const GitUtilsErrorCode = {
  NotValidUtf8S: 0,
	IOError: 1,
	NotValidUtf8F: 2,
	ReadFileError: 3,
	NoOwner: 4,
	GetStatusError: 5,
	IndexIsDetached: 6,
	RepoNotFound: 7,
	BlobNotFound: 8,
	RepoIsBare: 9,
	BranchNotFound: 10,
	SwitchBranchError: 11,
	CommitNotFound: 12,
	CurrentBranchNotFound: 13,
	HasConflicts: 14,
	UserUnConfigured: 15,
	UnStagedFile: 16,
	TreeNotFound: 17,
	RemoteNotFound: 18,
	BranchNotTrackAny: 19,
	SshAuthorizeError: 20,
	UserAuthorizeError: 21,
	RemoteHeadHasNotInLocal: 22,
	PushNeedNameAndPassword: 23,
	RepoAuthorNoConfig: 24,
	RepoHasConflicts: 25,
	NoStagedFile: 26,
	PushOtherError: 27,
	InvalidFilePath: 28,
	TargetReferenceNotDirect: 29,
	SwitchWillBeOverwrittenByMerge: 30,
	BuildMergeCommitError: 31,
	CommitBeforePullWouldBeOverwrittenByMerge: 32,
	CantPull: 33,
	OtherError: 34,
	Git2Error: 35,
	AnyhowError: 36
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
