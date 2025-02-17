import { getDirName } from "@/utils/tool";
import { Author, Repository } from ".";

export const emptyAuthor: Author = {
  name: '',
  email: ''
}

export const defaultRepository = (path: string): Repository => {
  return {
    id: 0,
    path: path,
    alias: getDirName(path) || path,
    hasWatch: true,
    order: 0,
    top: false
  }
}