import { BRANCH_COMMIT_CONTRIBUTION_KEY, SINGLE_REPO_EMIT } from "@/const/listen";
import { RepoStatus } from "@/enum";
import { Author, Branch, Commit, CommitFilter, CommitStatistic, DiffContent, CommitEntry, ChangedFile, FileHistoryItem, TreeDir, Repository } from "@/types";
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

type RepoPath = string
const bus = InvokeBus.getInstance()
export const addWatch = (repo: RepoPath) => {
  return bus.invoke('add_watch', { repo });
}

export const getDriver = () => {
  return bus.invoke('get_driver'); 
}

export const getSeparator = () => {
  return bus.invoke('get_separator');
}

export const getFolders = (path: string) => {
  return bus.invoke('get_folders', { path });
}

export const isRepo = (repo: RepoPath): Promise<boolean> => {
  return bus.invoke('is_repo', { repo });
}

export const workStatus = (repo: RepoPath) => {
  return bus.invoke<RepoStatus>('work_status', { repo })
}

export const setOwnership = (repo: RepoPath) => {
  return bus.invoke('set_repo_ownership', { repo }) 
}

export const removeWatch = (repo: RepoPath) => {
  return bus.invoke('remove_watch' , { repo })
}

export const getBranches = (repo: RepoPath) => {
  const branches = bus.invoke<Branch[]>('branches', { repo })
  return branches
}

export const getCurrentBranch = (repo: RepoPath) : Promise<Branch> => {
  return bus.invoke<Branch>('current_branch', { repo })
}

export const getBranchCommits = (repo: RepoPath, branch: Branch, count: Number) => {
  return bus.invoke<Commit[]>('branch_commits', { repo, branch, count }) 
}

export const beforeReferenceCommitsCount = (repo: RepoPath, reference: string) => {
  return bus.invoke<number>('before_reference_commits_count', { repo, reference }) 
}

export const reference_commit_filter_details = (repo: RepoPath, reference: string, filter: CommitFilter, offset?: number, count?: number) => {
  return bus.invoke<Commit[]>('reference_commit_filter_details', { repo, reference, filter, offset, count}) 
}

export const reference_commit_filter_count = (repo: RepoPath, reference: string, filter: CommitFilter, offset?: number, count?: number) => {
  return bus.invoke<number>('reference_commit_filter_count', { repo, reference, filter, offset, count  }) 
}

export const getAuthors = (repo: RepoPath, branch: Branch) => {
  return bus.invoke<Author[]>('authors', { repo, branch }) 
}

export const commitContent = (repo: RepoPath, cid: string) => {
  return bus.invoke<CommitEntry[]>('commit_content', { repo, cid })
}

export const getCommit = (repo: RepoPath, cid: string) => {
  return bus.invoke<Commit>('get_commit', { repo, cid})
}

export const fileDiff = (repo: RepoPath, old_id: string, new_id: string) => {
  return bus.invoke<DiffContent>('file_diff', { repo, old: old_id, 'new': new_id })
}

export const getBlobContent = (repo: RepoPath, cid: String) => {
  return bus.invoke<number[]>('blob_content', { repo, cid })
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
    bus.invoke('get_branch_commit_contribution', { key, repo, branch }) 
  })
}

export const getGlobalAuthor = () => {
  return bus.invoke<Author>('get_global_author')
}

export const getRepoAuthor = (repo: RepoPath) => {
  return bus.invoke<Author>('get_repo_author', { repo })  
}

export const getBranchCommitsAfterFilter = (repo: RepoPath, branch: Branch, filter: CommitFilter) => {
  return bus.invoke<Commit[]>('get_branch_commits_after_filter', { repo, branch, filter })
}

export const getChangedFiles = (repo: RepoPath) => {
  return bus.invoke<ChangedFile[]>('get_changed_files', { repo }) 
}

export const getStagedFiles = (repo: RepoPath) => {
  return bus.invoke<ChangedFile[]>('get_staged_files', { repo }) 
}

export const addFileToStage = (repo: RepoPath, path: string) => {
  return bus.invoke('add_to_stage', { repo, path }) 
}

export const removeFileFromStage = (repo: RepoPath, path: string) => {
  return bus.invoke('remove_from_stage', { repo, path }) 
}

export const checkoutFile = (repo: RepoPath, path: string) => {
  return bus.invoke('checkout_file', { repo, path }) 
}

export const commit = (repo: RepoPath, message: string, update_ref: string | undefined) => {
  return bus.invoke('commit', { repo, message, update_ref }) 
}

export const currentRemoteBranch = (repo: RepoPath) => {
  return bus.invoke<Branch>('current_remote_branch', { repo })
}

export const push = (repo: RepoPath, remote: string, branch: string, credentials:[string, String] | undefined) => {
  return bus.invoke('push', { repo, remote, branch, credentials }) 
}
export const pull = (repo: RepoPath, remote: string, branch: string, credentials:[string, String] | undefined) => {
  return bus.invoke('pull', { repo, remote, branch, credentials }) 
}

export const switchBranch = (repo: RepoPath, branch: Branch) => {
  return bus.invoke('switch_branch', { repo, branch })
}

/**
 * 提交一个仓库修改订阅，返回一个取消函数，调用取消函数可以取消提交
 * @param repo 
 * @returns 
 */
export const singleRepoSubmit = (repo: RepoPath, cb: () => void) => {
  bus.invoke('repo_single_submit', { repo })
  console.log(repo)
  let url = `${SINGLE_REPO_EMIT}:${repo.replace(/\\/g, '/')}`
  console.log(url)
  let unlisten = listen(url, (event) => {
    cb()
  })
  let unsubmit = () => {
    bus.invoke('repo_single_unsubmit', { repo })
    unlisten.then((unsub) => {
      unsub() 
    })
    unsubmit = () => undefined 
  }
  return unsubmit 
}

export const fileHistory = (repo: RepoPath, filePath: string) => {
  return bus.invoke<FileHistoryItem[]>('file_history', { repo, filePath }) 
}

export const commitTree = (repo: RepoPath, commitId: string) => {
  return bus.invoke<TreeDir>('get_commit_tree_recursive', {repo, commitId})
}

export const getTree = (repo: RepoPath, objectId: string, treePath: undefined | string = undefined) => {
  return bus.invoke<TreeDir>('get_tree', {repo, objectId, treePath})
}

export const objectIsBinary = (repo: RepoPath, objectId: string) => {
  return bus.invoke<boolean>('object_is_binary', {repo, objectId})
}

export const getRepoByPath = (repo: RepoPath) => {
  return bus.invoke<Repository>('get_repo_by_path', {path: repo})
}

export const saveBlob = (repo: RepoPath, objectId: string, path: string) => {
  return bus.invoke('save_blob', {repo, objectId, path})
}
