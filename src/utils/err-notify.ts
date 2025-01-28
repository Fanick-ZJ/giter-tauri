import { useNotificationStore } from "@/store/modules/notification";
import { NotificationBody, NotificationLevel, NotificationType } from "@/store/modules/notification/type";
import { Error } from "@/types";
import { useNotification } from "naive-ui";
import { setOwnership } from "./command";

const RepoOwnershipError = (err: Error, okCb?: () => void): NotificationBody => {
  const key = `${NotificationType.Owner}:${err.data}`
  return {
    title: '仓库所有权错误',
    body: err.data,
    type: NotificationLevel.Error,
    key: key as NotificationBody['key'],
    cb: {
      name: '获取所有权',
      cb: (): Promise<boolean | void> => {
        const store = useNotificationStore()
        store.remove(key)
        return setOwnership(err.data)
        .then(() => {
          okCb && okCb()
         return true 
        }).catch((e) => {
          cmdErrNotify(e)
        })
      }
    }
  }
}

export const cmdErrNotify = (err: Error, okCb?: () => void, immediatly=false) => {
  let notify = undefined;
  if (err.type === 'RepoHasnotOwnership') {
    notify = RepoOwnershipError(err, okCb)
  }
  if (notify != undefined) {
   const store = useNotificationStore()
   store.add(notify)
  }
  if (immediatly && notify) {
   useNotification().error(notify)
  }
}