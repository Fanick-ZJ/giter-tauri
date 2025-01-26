export enum SetupStoreId {
  App = 'app-store',
  Theme = 'theme-store',
  Repo = 'repo-store'
}

export const RepoStatus = {
  Ok: 1,
  Modified: 1 << 2,
  Untracked: 1 << 3,
  Uncommitted: 1 << 4,
  Unpushed: 1 << 5,
} as const;

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