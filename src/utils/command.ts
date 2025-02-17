import { ADD_WATCH, BLOB_CONTENT, COMMIT_CONTENT, FILE_DIFF, GET_AUTHORS, GET_BRANCHES, GET_COMMIT, GET_BRANCH_COMMITS, GET_CURRENT_BRANCH, GET_DRIVER, GET_FOLDERS, GET_SEPARATOR, IS_REPO, REMOVE_WATCH, SET_OWNERSHIP, WORK_STATUS, BRANCH_COMMIT_CONTRIBUTION } from "@/const/command";
import { BRANCH_COMMIT_CONTRIBUTION_KEY } from "@/const/listen";
import { RepoStatus } from "@/enum";
import { Author, Branch, Commit, CommitFilter, CommitStatistic, DiffContent, File } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type RepoPath = string
export const addWatch = (repo: RepoPath) => {
  return invoke(ADD_WATCH, { repo });
}

export const getDriver = () => {
  return invoke(GET_DRIVER); 
}

export const getSeparator = () => {
  return invoke(GET_SEPARATOR);
}

export const getFolders = (path: string) => {
  return invoke(GET_FOLDERS, { path });
}

export const isRepo = (repo: RepoPath) => {
  return invoke(IS_REPO, { repo });
}

export const workStatus = (repo: RepoPath) => {
  const status = invoke(WORK_STATUS, { repo })
  return status
}

export const setOwnership = (repo: RepoPath) => {
  return invoke(SET_OWNERSHIP, { repo }) 
}

export const removeWatch = (repo: RepoPath) => {
  return invoke(REMOVE_WATCH , { repo })
}

export const getBranches = (repo: RepoPath) => {
  const branches = invoke<Branch[]>(GET_BRANCHES, { repo })
  return branches
}

export const getCurrentBranch = (repo: RepoPath) : Promise<Branch> => {
  return invoke<Branch>(GET_CURRENT_BRANCH, { repo })
}

export const getBranchCommits = (repo: RepoPath, branch: Branch, count: Number) => {
  return invoke<Commit[]>(GET_BRANCH_COMMITS, { repo, branch, count }) 
}

export const getAuthors = (repo: RepoPath, branch: Branch) => {
  return invoke<Author[]>(GET_AUTHORS, { repo, branch }) 
}

export const commitContent = (repo: RepoPath, cid: string) => {
  return invoke<File[]>(COMMIT_CONTENT, { repo, cid })
}

export const getCommit = (repo: RepoPath, cid: string) => {
  return invoke<Commit>(GET_COMMIT, { repo, cid})
}

export const fileDiff = (repo: RepoPath, old_id: string, new_id: string) => {
  return invoke<DiffContent>(FILE_DIFF, { repo, old: old_id, 'new': new_id })
}

export const getBlobContent = (repo: RepoPath, cid: string) => {
  return invoke<number[]>(BLOB_CONTENT, { repo, cid })
}

export const getBranchCommitContribution = (repo: RepoPath, branch: Branch): Promise<CommitStatistic[]> => {
  return new Promise((resolve, reject) => {
    const key = Date.now().toString()
    const unsubscribe = listen(BRANCH_COMMIT_CONTRIBUTION_KEY(key), (event) => {
      if (event.payload instanceof String) {
        reject(event.payload)
      }
      resolve(event.payload as CommitStatistic[])
      unsubscribe.then((unsub) => {
        unsub() 
      })
    })
    invoke(BRANCH_COMMIT_CONTRIBUTION, { key, repo, branch }) 
  })
}

export const getGlobalAuthor = () => {
  return invoke<Author>('get_global_author')
}

export const getRepoAuthor = (repo: RepoPath) => {
  return invoke<Author>('get_repo_author', { repo })  
}

export const getBranchCommitsAfterFilter = (repo: RepoPath, branch: Branch, filter: CommitFilter) => {
  return invoke<Commit[]>('get_branch_commits_after_filter', { repo, branch, filter })
}