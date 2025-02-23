import { RepoStatus } from "@/enum"

export const STATUS_CHANGE = 'giter://status_changed'
export type StatusChangePayloadType = {
  path: string,
  status: RepoStatus,
}
// 获取分支的贡献度，key为键值，不允许重复，用于区分不同的监听事件，这个监听是一次性的
export const BRANCH_COMMIT_CONTRIBUTION_KEY = (key: String) => `giter://branch_contribution/${key}`
export const CHANGED_EMIT = 'giter://changed_emit'