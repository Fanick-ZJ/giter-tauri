import { ADD_WATCH, GET_DRIVER, GET_FOLDERS, GET_SEPARATOR, IS_REPO, SET_OWNERSHIP, WORK_STATUS } from "@/const/command";
import { RepoStatus } from "@/enum";
import { CommandError } from "@/enum/error";
import { invoke } from "@tauri-apps/api/core";

export const add_watch = async (path: string) => {
  return await invoke(ADD_WATCH, { path });
}

export const get_driver = async () => {
  return await invoke(GET_DRIVER); 
}

export const get_separator = async () => {
  return await invoke(GET_SEPARATOR);
}

export const get_folders = async (path: string) => {
  return await invoke(GET_FOLDERS, { path });
}

export const is_repo = async (path: string) => {
  return await invoke(IS_REPO, { path });
}

export const work_status = async (path: string) => {
  const status = await invoke(WORK_STATUS, { path })
  return status as RepoStatus 
}

export const set_ownership = async (path: string) => {
  return await invoke(SET_OWNERSHIP, { path }) 
}