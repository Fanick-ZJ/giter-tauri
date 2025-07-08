export enum SetupStoreId {
  App = 'app-store',
  Theme = 'theme-store',
  Repo = 'repo-store',
  Notification = 'notify-store',
}

export const RepoStatus = {
  Ok: 1,
  Modified: 1 << 2,
  Untracked: 1 << 3,
  Uncommitted: 1 << 4,
  Unpushed: 1 << 5,
} as const;

export enum FileStatus {
  Added = 'Added',
  Deleted = 'Deleted',
  Modified = 'Modified',
  Renamed = 'Renamed',
  Conflicted = 'Conflicted',
  Ok = 'Ok',
}

export enum TreeFileMode {
  Unreadable = 0o000000,
  Tree = 0o040000,
  Blob = 0o100644,
  BlobGroupWritable = 0o100664,
  BlobExecutable = 0o100755,
  Link = 0o120000,
  Commit = 0o160000,
}

export type RepoStatus = typeof RepoStatus[keyof typeof RepoStatus];

export const hasFlag = (status: RepoStatus, flag: RepoStatus) => {
  return (status & flag) === flag;
}

export const parseStatus = (status: RepoStatus) => {
  const res: RepoStatus[] = [];
  Object.values(RepoStatus).forEach((flag) => {
    if (hasFlag(status, flag)) {
      res.push(flag);
    } 
  })
  return res;
}