import { invoke } from "@tauri-apps/api/core"
import { GET_SEPARATOR } from "./command"

export const SEPERATOR = await invoke(GET_SEPARATOR, {}) as string

