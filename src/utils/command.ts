import { ADD_WATCH, BLOB_CONTENT, COMMIT_CONTENT, FILE_DIFF, GET_AUTHORS, GET_BRANCHES, GET_COMMIT, GET_BRANCH_COMMITS, GET_CURRENT_BRANCH, GET_DRIVER, GET_FOLDERS, GET_SEPARATOR, IS_REPO, REMOVE_WATCH, SET_OWNERSHIP, WORK_STATUS, BRANCH_COMMIT_CONTRIBUTION, GET_GLOBAL_AUTHOR, GET_REPO_AUTHOR, GET_BRANCH_COMMITS_AFTER_FILTER, GET_CHANGED_FILES, GET_STAFED_FILES } from "@/const/command";
import { BRANCH_COMMIT_CONTRIBUTION_KEY } from "@/const/listen";
import { Author, Branch, Commit, CommitFilter, CommitStatistic, DiffContent, CommitFile, ChangedFile } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type InvokeUnit = {
  id: number,
  command: string,
  args: any,
  resolve: (value: any) => void,
  reject: (reason: any) => void,
  startTime: number,
}

// 事件执行单例总线，用于执行事件、流量限制等
class InvokeBus {
  private taskCount = 0;
  private static instance: InvokeBus;
  private invokePool: InvokeUnit[] = [];
  private flowMaxSize = 20;
  private flow: number = 0;
  private constructor() { }
  public static getInstance(): InvokeBus {
    if (!InvokeBus.instance) {
      InvokeBus.instance = new InvokeBus();
    }
    return InvokeBus.instance;
  }

  public async invoke<T>(command: string, args?: any): Promise<T> {
    if (this.flow >= this.flowMaxSize) {
      // 返回一个Promise，将其放入invokePool中，等待其他Promise执行完毕后再执行
      let resolve: (value: T) => void;
      let reject: (reason: any) => void;
      const promise = new Promise<T>((_resolve, _reject) => {
        resolve = _resolve;
        reject = _reject;
        this.taskCount++
        this.invokePool.push({
          id: this.taskCount,
          command,
          args,
          resolve,
          reject,
          startTime: Date.now(),
        })
      })
      return promise
    } else {
      // 直接执行
      this.flow++
      return invoke<T>(command, args).finally(() => {
        this.flow--
        this.next() 
      })
    }
  }

  private next() {
    if (this.flow >= this.flowMaxSize) {
      return
    }
    if (this.invokePool.length > 0) {
      const invokeUnit = this.invokePool.shift()
      if (invokeUnit) {
        this.flow++
        invoke(invokeUnit.command, invokeUnit.args).then((res) => {
          invokeUnit.resolve(res)
        }).catch((err) => {
          invokeUnit.reject(err) 
        }).finally(() => {
          this.flow--
          this.next()
          // console.log(`${invokeUnit.id} invoke ${invokeUnit.command} cost ${Date.now() - invokeUnit.startTime}ms , has ${this.invokePool.length} tasks in queue`)
        })
      } 
    } 
  }

  setFlowMaxSize(size: number) {
    this.flowMaxSize = size 
  }
  
}

type RepoPath = String
const bus = InvokeBus.getInstance()
export const addWatch = (repo: RepoPath) => {
  return bus.invoke(ADD_WATCH, { repo });
}

export const getDriver = () => {
  return bus.invoke(GET_DRIVER); 
}

export const getSeparator = () => {
  return bus.invoke(GET_SEPARATOR);
}

export const getFolders = (path: string) => {
  return bus.invoke(GET_FOLDERS, { path });
}

export const isRepo = (repo: RepoPath) => {
  return bus.invoke(IS_REPO, { repo });
}

export const workStatus = (repo: RepoPath) => {
  return bus.invoke(WORK_STATUS, { repo })
}

export const setOwnership = (repo: RepoPath) => {
  return bus.invoke(SET_OWNERSHIP, { repo }) 
}

export const removeWatch = (repo: RepoPath) => {
  return bus.invoke(REMOVE_WATCH , { repo })
}

export const getBranches = (repo: RepoPath) => {
  const branches = bus.invoke<Branch[]>(GET_BRANCHES, { repo })
  return branches
}

export const getCurrentBranch = (repo: RepoPath) : Promise<Branch> => {
  return bus.invoke<Branch>(GET_CURRENT_BRANCH, { repo })
}

export const getBranchCommits = (repo: RepoPath, branch: Branch, count: Number) => {
  return bus.invoke<Commit[]>(GET_BRANCH_COMMITS, { repo, branch, count }) 
}

export const getAuthors = (repo: RepoPath, branch: Branch) => {
  return bus.invoke<Author[]>(GET_AUTHORS, { repo, branch }) 
}

export const commitContent = (repo: RepoPath, cid: string) => {
  return bus.invoke<CommitFile[]>(COMMIT_CONTENT, { repo, cid })
}

export const getCommit = (repo: RepoPath, cid: string) => {
  return bus.invoke<Commit>(GET_COMMIT, { repo, cid})
}

export const fileDiff = (repo: RepoPath, old_id: string, new_id: string) => {
  return bus.invoke<DiffContent>(FILE_DIFF, { repo, old: old_id, 'new': new_id })
}

export const getBlobContent = (repo: RepoPath, cid: String) => {
  return bus.invoke<number[]>(BLOB_CONTENT, { repo, cid })
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
    bus.invoke(BRANCH_COMMIT_CONTRIBUTION, { key, repo, branch }) 
  })
}

export const getGlobalAuthor = () => {
  return bus.invoke<Author>(GET_GLOBAL_AUTHOR)
}

export const getRepoAuthor = (repo: RepoPath) => {
  return bus.invoke<Author>(GET_REPO_AUTHOR, { repo })  
}

export const getBranchCommitsAfterFilter = (repo: RepoPath, branch: Branch, filter: CommitFilter) => {
  return bus.invoke<Commit[]>(GET_BRANCH_COMMITS_AFTER_FILTER, { repo, branch, filter })
}

export const getChangedFiles = (repo: RepoPath) => {
  return bus.invoke<ChangedFile[]>(GET_CHANGED_FILES, { repo }) 
}

export const getStagedFiles = (repo: RepoPath) => {
  return bus.invoke<ChangedFile[]>(GET_STAFED_FILES, { repo }) 
}