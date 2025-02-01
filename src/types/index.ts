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

declare global {
  interface Window {
    $message: MessageApiInjection;
    $notification: NotificationApiInjection;
  }
}