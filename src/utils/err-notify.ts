import { createNofication } from "@/components/repo-home/components/notification";
import { SET_OWNERSHIP } from "@/const/command";
import { useNotificationStore } from "@/store/modules/notification";
import { NotificationBody, NotificationLevel, NotificationType } from "@/store/modules/notification/type";
import { Error } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { useNotification } from "naive-ui";

const RepoOwnershipError = (err: Error): NotificationBody => {
  const key = `${NotificationType.Owner}:${err.data}`
  return {
    title: '仓库所有权错误',
    body: err.data,
    type: NotificationLevel.Error,
    key: key as NotificationBody['key'],
    cb: {
      name: '获取所有权',
      cb: () => {
        const store = useNotificationStore()
        store.remove(key)
        return invoke(SET_OWNERSHIP, { path: err.data })
        .then(() => {
         return true 
        })
        .catch((e) => {
          cmdErrNotify(e)
        })
      }
    }
  }
}

export const cmdErrNotify = (err: Error, immediatly=false) => {
  let notify = undefined;
  if (err.type === 'RepoHasnotOwnership') {
    notify = RepoOwnershipError(err)
  }
  if (notify != undefined) {
   const store = useNotificationStore()
   store.add(notify)
  }
  if (immediatly && notify) {
   useNotification().error(notify)
  }
}