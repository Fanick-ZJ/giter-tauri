import { FileStatus, EntryMode as EntryMode } from "@/enum";
import { CommonErrorCode } from "@/enum/error"
import { MessageApiInjection } from "naive-ui/es/message/src/MessageProvider";
import { NotificationApiInjection } from "naive-ui/es/notification/src/NotificationProvider";

export type Repository = {
  id: number
  path: string
  alias: string
  hasWatch: boolean
  order: number
  top: boolean
}

export type Error = {
  type: keyof typeof CommonErrorCode,
  data: string
}

export type Author = {
  name: string,
  email: string 
}

export type Branch = {
 name: string,
 isRemote: boolean
 reference: string 
}

export type Commit = {
  commitId: string,
  authorName: string,
  authorEmail: string,
  committerName: string,
  committerEmail: string,
  title: string,
  message: string,
  datetime: number,
  parents: string[],
  repo: string
}


export type CommitEntry = {
  path: string
  status: FileStatus
  objectId: string
  entryMode: EntryMode,
  prevObjectId: string
  prevEntryMode: EntryMode,
}

export type ChangedFile = {
  path: string
  status: FileStatus
  prevObjectId: string
}

export type Diff = {
  oldPath: string
  newPath: string
  status: FileStatus 
}

export type DiffOpt = {
  op: 'equal' | 'replace' | 'insert' | 'delete',
  old_index: number,
  old_len: number,
  new_index: number,
  new_len: number,
  len: number
}

export type DiffContent = {
  oldContent: string,
  newContent: string,
  ops: DiffOpt[],
  display: string
}
export type YMDStr = `${number}-${number}-${number}`
export type CommitStatistic = {
  repo: string,
  branch: Branch,
  author: Author,
  stats: {[key in YMDStr]: number}[]
}

export type CommitFilter = {
  lastId?: string,
  author?: Author,
  startTime?: number,
  endTime?: number,
  message?: string,
}

export type FileHistoryItem = {
  commit: Commit,
  file: CommitEntry,
}

export type TreeEntryMetadata = {
  size: number,
  mode: EntryMode,
  object_id: string
}

export type TreeFile = {
  name: string,
  path: string,
  metadata: TreeEntryMetadata
  type: 'file'
}

export type TreeDir = {
  path: string,
  name: string,
  children: (TreeFile | TreeDir)[],
  metadata: TreeEntryMetadata
  type: 'dir'
}

declare global {
  interface Window {
    $message: MessageApiInjection;
    $notification: NotificationApiInjection;
  }
}