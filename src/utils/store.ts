import { Repository } from "@/types/store"
import { get_store_db } from "./storage"

// 根据id获取仓库
export const getRepositoryById = async (id: number) => {
  const db = await get_store_db()
  const res = await db.select<[Repository]>(`SELECT * FROM repository WHERE id = ${id}`)
  return res.length > 0 ? res[0] : undefined
}

// 根据路径获取仓库
export const getRepositoryByPath = async (path: string) => {
  const db = await get_store_db()
  const res = await db.select<[Repository]>(`SELECT * FROM repository WHERE path = '${path}'`)
  return res.length > 0? res[0] : undefined
}