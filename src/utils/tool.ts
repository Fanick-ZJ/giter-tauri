import { SEPERATOR } from "@/const"

export const getDirName = (path: string) => {
  return path.split(SEPERATOR).pop()
}