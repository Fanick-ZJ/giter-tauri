import { ADD_WATCH, COMMIT_CONTENT, FILE_DIFF, GET_AUTHORS, GET_BRANCHES, GET_COMMITS, GET_CURRENT_BRANCH, GET_DRIVER, GET_FOLDERS, GET_SEPARATOR, IS_REPO, REMOVE_WATCH, SET_OWNERSHIP, WORK_STATUS } from "@/const/command";
import { RepoStatus } from "@/enum";
import { Author, Branch, Commit, DiffContent, File } from "@/types";
import { invoke } from "@tauri-apps/api/core";

type RepoPath = string
export const addWatch = async (repo: RepoPath) => {
  return await invoke(ADD_WATCH, { repo });
}

export const getDriver = async () => {
  return await invoke(GET_DRIVER); 
}

export const getSeparator = async () => {
  return await invoke(GET_SEPARATOR);
}

export const getFolders = async (path: string) => {
  return await invoke(GET_FOLDERS, { path });
}

export const isRepo = async (repo: RepoPath) => {
  return await invoke(IS_REPO, { repo });
}

export const workStatus = async (repo: RepoPath) => {
  const status = await invoke(WORK_STATUS, { repo })
  return status as RepoStatus 
}

export const setOwnership = async (repo: RepoPath) => {
  return await invoke(SET_OWNERSHIP, { repo }) 
}

export const removeWatch = async (repo: RepoPath) => {
  return await invoke(REMOVE_WATCH , { repo })
}

export const getBranches = async (repo: RepoPath) => {
  const branches = await invoke<Branch[]>(GET_BRANCHES, { repo })
  return branches
}

export const getCurrentBranch = async (repo: RepoPath) : Promise<Branch> => {
  return await invoke<Branch>(GET_CURRENT_BRANCH, { repo })
}

export const getBranchCommits = async (repo: RepoPath, branch: Branch, count: Number) => {
  return await invoke<Commit[]>(GET_COMMITS, { repo, branch, count }) 
}

export const getAuthors = async (repo: RepoPath, branch: Branch) => {
  return await invoke<Author[]>(GET_AUTHORS, { repo, branch }) 
}

export const commitContent = async (repo: RepoPath, cid: string) => {
  return await invoke<File[]>(COMMIT_CONTENT, { repo, cid })
}

export const fileDiff = async (repo: RepoPath, old_id: string, new_id: string) => {
  return await invoke<DiffContent>(FILE_DIFF, { repo, old: old_id, 'new': new_id })
}