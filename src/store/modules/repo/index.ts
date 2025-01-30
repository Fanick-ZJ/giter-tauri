import { STATUS_CHANGE } from "@/const/listen";
import { RepoStatus, SetupStoreId } from "@/enum";
import { Repository } from "@/types";
import { addWatch, isRepo, removeWatch, workStatus } from "@/utils/command";
import { cmdErrNotify } from "@/utils/err-notify";
import { get_store_db } from "@/utils/storage";
import { readRepos, removeRepo, saveRepo, updateRepo } from "@/utils/store";
import { listen } from "@tauri-apps/api/event";
import { defineStore } from "pinia";
import { Ref, ref } from "vue";

type RepoPath = string
export type ValidRepository = Repository & { valid: boolean }
export const useRepoStore = defineStore(SetupStoreId.Repo, () => {
  const repos = ref<ValidRepository[]>([])
  const status: Map<RepoPath, Ref<RepoStatus>> = new Map()

  const _init_opt = (repo: ValidRepository) => {
    isRepo(repo.path).then(is => {
      if (!is){
        repo.valid = false
        return
      }
      if (!repo.hasWatch) {
        return
      }
      addWatch(repo.path)
      status.set(repo.path, ref(RepoStatus.Ok))
      workStatus(repo.path).then((res) => {
        setStatus(repo.path, res) 
      }).catch((err) => {
        cmdErrNotify(err, () => workStatus(repo.path))
      })
    })
    .then(() => {
      repos.value.push(repo)
      repos.value.sort(repoSort) 
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
    readRepos().then((res: string | any[]) => {
      for (let i = 0; i < res.length; i++) {
        const repo = res[i];
        repo.hasWatch = !!repo.hasWatch
        repo.top = !!repo.top
        Object.assign(repo, {
          valid: true,
        })
        _init_opt(repo as ValidRepository)
      }
    })
  }
  // 读取仓库信息
  init_repo()


  // 添加仓库
  const add = (repo: Repository) => {
    saveRepo(repo).then(() => {
      Object.assign(repo, {
        valid: true,
      })
      // @ts-ignore
      _init_opt(repo)
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
    repos,
    add,
    status,
    update,
    getRepoByPath,
    remove
  }
})