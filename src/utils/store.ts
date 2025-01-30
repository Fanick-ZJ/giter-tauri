import { Repository } from "@/types"
import { get_store_db } from "./storage"

// 根据id获取仓库
export const getRepositoryById = async (id: number) => {
  const db = await get_store_db()
  const res = await db.select<Repository[]>(`SELECT * FROM repository WHERE id = ${id}`)
  if (res.length === 0) {
    return undefined 
  }
  let repo = res[0]
  // @ts-ignore
  repo.hasWatch =!!repo.has_watch
  repo.top =!!repo.top
  return repo
}

// 根据路径获取仓库
export const getRepositoryByPath = async (path: string) => {
  const db = await get_store_db()
  const res = await db.select<[Repository]>(`SELECT * FROM repository WHERE path = '${path}'`)
  return res.length > 0? res[0] : undefined
}

export const saveRepo = async (repo: Repository) => {
  const db = await get_store_db()
  return db.execute('INSERT INTO repository (path, alias, has_watch, `order`, top) VALUES (?, ?, ?, ?, ?)', 
    [repo.path, repo.alias, Number(repo.hasWatch), repo.order, Number(repo.top)])
}

export const readRepos = async () => {
  const db = await get_store_db()
  return db.select<[Repository]>('SELECT id, path, alias, has_watch as hasWatch, `order`, top FROM repository order by top desc, `order`, alias')
}

export const updateRepo = async (repo: Repository) => {
  const db = await get_store_db()
  return db.execute('UPDATE repository SET path =?, alias =?, has_watch =?, `order` =?, top =? WHERE id =?',
    [repo.path, repo.alias, Number(repo.hasWatch), repo.order, Number(repo.top), repo.id]) 
}

export const removeRepo = async (id: number) => {
  const db = await get_store_db()
  return db.execute('DELETE FROM repository WHERE id =?', [id]) 
}