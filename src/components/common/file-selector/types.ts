export type SelectFilter = ((path: string) => boolean)

// 带T_的类型，都是由后端返回的类型，不一定与前端一致
export type T_Dir = {
  name: string,
  path: string,
  is_repo: boolean,
}

export type Folder = {
  name: string,
  path: string,
  children?: Folder[],
  is_repo: boolean,
}

export type FileSelectorOptions = {
  path?: string
  multiple?: boolean
  directory?: boolean
  filter?: SelectFilter
  repoTip?: boolean
  root?: HTMLElement
}
