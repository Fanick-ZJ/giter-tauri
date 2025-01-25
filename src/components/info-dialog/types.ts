export type SelectFilter = ((path: string) => boolean)

export type RepoInfoProps = {
  path?: string
  id?: number,
  mode: 'add' | 'edit'
}