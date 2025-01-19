import { SetupStoreId } from "@/enum";
import { Repository } from "@/types/store";
import { get_store_db } from "@/utils/storage";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useRepoStore = defineStore(SetupStoreId.Repo, () => {
  const repos = ref<Repository[]>([])

  const read_repos = async () => {
    const db = get_store_db()
    db.then(db => {
      db.select<[Repository]>('SELECT * FROM repos').then((res) => {
        for (let i = 0; i < res.length; i++) {
          const repo = res[i];
          repos.value.push(repo)
        }
      })
    })
  }
  read_repos()

  const save_repo = async (repo: Repository) => {
    const db = get_store_db()
    db.then(db => {
      db.execute('INSERT INTO repos (name, alias, has_watch, order, top) VALUES (?, ?, ?, ?, ?)', [repo.name, repo.alias, repo.hasWatch, repo.order, repo.top]).then(() => {
        repos.value.push(repo)
      })
    })
  }

  const add = (repo: Repository) => {
    repos.value.push(repo)
    save_repo(repo)
  }
  return {
    repos,
    add
  }
})