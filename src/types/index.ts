import { CommandError } from "@/enum/error"
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
  type: keyof typeof CommandError,
  data: string
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
  parent_count: number,
  repo: string
}

declare global {
  interface Window {
    $message: MessageApiInjection;
    $notification: NotificationApiInjection;
  }
}