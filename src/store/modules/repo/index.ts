import { STATUS_CHANGE } from "@/const/listen";
import { RepoStatus, SetupStoreId } from "@/enum";
import { Repository } from "@/types";
import { addWatch, isRepo, removeWatch, workStatus } from "@/utils/command";
import { readRepos, removeRepo, saveRepo, updateRepo } from "@/utils/store";
import { listen } from "@tauri-apps/api/event";
import { QueryResult } from "@tauri-apps/plugin-sql";
import { defineStore } from "pinia";
import { Ref, ref } from "vue";

type RepoPath = string
export type ValidRepository = Repository & { valid: boolean }
export const useRepoStore = defineStore(SetupStoreId.Repo, () => {
  const repos = ref<ValidRepository[]>([])
  const status: Map<RepoPath, Ref<RepoStatus>> = new Map()

  const _init_opt = async (repo: ValidRepository) => {
    return isRepo(repo.path).then(async is => {
      if (!is){
        repo.valid = false
        return repo
      }
      if (repo.hasWatch) {
        addWatch(repo.path) 
      }
      status.set(repo.path, ref(RepoStatus.Ok))
      try {
        const _status = await workStatus(repo.path) 
        repo.valid = true
        setStatus(repo.path, _status as RepoStatus)
      } catch (error) {
        window.$message.error('获取仓库状态失败')
        repo.valid = false
      }
      return repo
    }).catch((err) => {
      window.$message.error('获取仓库状态失败')
      return repo
    })
  }

  const repoSort = (a: Repository, b: Repository) => {
    // 按 top 降序
    if (a.top < b.top) return 1;
    if (a.top > b.top) return -1;
  
    // 如果 top 相同，则按 order 升序排序
    if (a.order < b.order) return -1;
    if (a.order > b.order) return 1;
  
    // 如果 top 和 order 都相同，则按 alias 升序排序
    if (a.alias < b.alias) return -1;
    if (a.alias > b.alias) return 1;
  
    // 如果所有属性都相同，返回 0
    return 0;
  }

  const init_repo = async () => {
    const __repos = await readRepos()
    const repoInitPromise:Promise<ValidRepository>[] = []
    for (let i = 0; i < __repos.length; i++) {
      const repo = __repos[i];
      repo.hasWatch = !!repo.hasWatch
      repo.top = !!repo.top
      const validRepo: ValidRepository = {
        ...repo,
        valid: false 
      }
      repoInitPromise.push(_init_opt(validRepo))
    }
    await Promise.all(repoInitPromise).then((res) => {
      repos.value = res.sort(repoSort)
    })
  }

  // 添加仓库
  const add = (repo: Repository) => {
    saveRepo(repo).then((res: QueryResult) => {
      repo.id = res.lastInsertId as number
      const validRepo: ValidRepository = {
        ...repo,
        valid: false 
      }
      // @ts-ignore
      _init_opt(repo).then((res) => {
        repos.value.push(res)
        repos.value.sort(repoSort) 
      })
    })
  }

  // 监听仓库状态变化
  listen(STATUS_CHANGE, (event) => {
    const { path, status } = event.payload as { path: string, status: RepoStatus }
    console.log('status change', path, status)
    setStatus(path, status)
  })


  // 设置仓库状态
  const setStatus = (path: RepoPath, _status: RepoStatus) => {
    if (status.has(path)) {
      status.get(path)!.value = _status
    }
  }

  // 更新仓库信息
  const update = async (repo: Repository) => {
    updateRepo(repo)
    const index = repos.value.findIndex((r) => r.id === repo.id)
    if (index == -1) return
    const _repo = repos.value[index]
    const is = await isRepo(repo.path) as boolean
    _repo.valid = is
    Object.assign(repos.value[index], repo)
    repos.value.sort(repoSort)
  }

  const getRepoById = (id: number) => {
    return repos.value.find((repo) => repo.id === id) 
  }

  const getRepoByPath = (path: RepoPath) => {
    return repos.value.find((repo) => repo.path === path) 
  }

  const remove = (repo: ValidRepository) => {
    removeRepo(repo.id)
    removeWatch(repo.path)
    const index = repos.value.findIndex((r) => r.id === repo.id)
    if (index == -1) return
    repos.value.splice(index, 1)
  }

  return {
    init_repo,
    repos,
    add,
    status,
    update,
    getRepoByPath,
    getRepoById,
    remove
  }
})