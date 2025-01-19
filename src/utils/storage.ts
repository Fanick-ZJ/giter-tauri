import { invoke } from "@tauri-apps/api/core"
import Database from '@tauri-apps/plugin-sql';

type DbName = 'store' | 'cache' | 'config'

const get_db = async (db: DbName): Promise<Database> => {
  let path = await invoke('get_db_path', { db })
  return Database.load(`sqlite:${path}`)
}

export const get_store_db = async () => {
  return get_db('store')
}

export const get_cache_db = async () => {
  return get_db('cache')
}

export const get_config_db = async () => {
  return get_db('config')
}