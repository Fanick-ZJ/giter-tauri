import { ADD_WATCH } from "@/const/command";
import { STATUS_CHANGE } from "@/const/listen";
import { RepoStatus, SetupStoreId } from "@/enum";
import { Repository } from "@/types/store";
import { get_store_db } from "@/utils/storage";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { defineStore } from "pinia";
import { Ref, ref } from "vue";

const watch = async (path: string) => {
  invoke(ADD_WATCH, { path }).then(() => {
    console.log('add watch', path)
  })
}

type RepoPath = string
export const useRepoStore = defineStore(SetupStoreId.Repo, () => {
  const repos = ref<Repository[]>([])
  const status: Map<RepoPath, Ref<RepoStatus[]>> = new Map()

  const _init_opt = (repo: Repository) => {
    repos.value.push(repo)
    watch(repo.path)
    status.set(repo.path, ref([]))
  }

  const read_repos = async () => {
    const db = get_store_db()
    db.then(db => {
      db.select<[Repository]>('SELECT * FROM repository').then((res) => {
        for (let i = 0; i < res.length; i++) {
          const repo = res[i];
          _init_opt(repo)
        }
      })
    })
  }
  read_repos()

  const save_repo = async (repo: Repository) => {
    const db = await get_store_db()
    return db.execute('INSERT INTO repository (path, alias, has_watch, `order`, top) VALUES (?, ?, ?, ?, ?)', 
      [repo.path, repo.alias, repo.hasWatch, repo.order, repo.top])
  }

  // 添加仓库
  const add = (repo: Repository) => {
    save_repo(repo).then(() => {
      _init_opt(repo)
    })
  }

  // 监听仓库状态变化
  listen(STATUS_CHANGE, (event) => {
    const { path, status } = event.payload as { path: string, status: RepoStatus[] }
    console.log('listen', path, status)
    setStatus(path, status)
  })


  // 设置仓库状态
  const setStatus = (path: RepoPath, _status: RepoStatus[]) => {
    console.log('status change', path, _status)
    if (status.has(path)) {
      status.get(path)!.value = _status
    }
  }
  return {
    repos,
    add,
    status,
  }
})