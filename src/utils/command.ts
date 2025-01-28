import { ADD_WATCH, GET_DRIVER, GET_FOLDERS, GET_SEPARATOR, IS_REPO, SET_OWNERSHIP, WORK_STATUS } from "@/const/command";
import { RepoStatus } from "@/enum";
import { CommandError } from "@/enum/error";
import { invoke } from "@tauri-apps/api/core";

export const addWatch = async (path: string) => {
  return await invoke(ADD_WATCH, { path });
}

export const getDriver = async () => {
  return await invoke(GET_DRIVER); 
}

export const getSeparator = async () => {
  return await invoke(GET_SEPARATOR);
}

export const getFolders = async (path: string) => {
  return await invoke(GET_FOLDERS, { path });
}

export const isRepo = async (path: string) => {
  return await invoke(IS_REPO, { path });
}

export const workStatus = async (path: string) => {
  const status = await invoke(WORK_STATUS, { path })
  return status as RepoStatus 
}

export const setOwnership = async (path: string) => {
  return await invoke(SET_OWNERSHIP, { path }) 
}