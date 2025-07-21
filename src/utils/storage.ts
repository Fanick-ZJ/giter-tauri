import { GET_DB_PATH } from "@/const/command";
import { invoke } from "@tauri-apps/api/core"
import Database from '@tauri-apps/plugin-sql';

type DbName = 'store' | 'cache' | 'config'

const get_db = async (db: DbName): Promise<Database> => {
  let path = await invoke(GET_DB_PATH, { db })
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

export const setLocalStage = async (key: string, value: string) => {
  window.localStorage.setItem(key, value)
}

export const LOCAL_STORAGE_DICT = {
  LAST_SAVE_PATH: 'LAST_SAVE_PATH',
}
export const getLocalStage = (key: keyof typeof LOCAL_STORAGE_DICT) => {

  return window.localStorage.getItem(LOCAL_STORAGE_DICT[key])
}

getLocalStage('LAST_SAVE_PATH')