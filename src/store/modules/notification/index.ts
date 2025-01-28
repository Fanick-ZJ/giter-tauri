import { SetupStoreId } from "@/enum";
import { defineStore } from "pinia";
import { NotificationBody } from "./type";
import { ref } from "vue";

export const useNotificationStore = defineStore(SetupStoreId.Notification, () => {
  const notifications = ref<NotificationBody[]>([])

  const add = (message: NotificationBody) => {
    // 同一个key的消息只会存在一个
    const index = notifications.value.findIndex((item) => item.key === message.key)
    if (index !== -1) {
      notifications.value.splice(index, 1)
    }
    notifications.value.push(message) 
  }

  const remove = (key: String) => {
    const index = notifications.value.findIndex((item) => item.key === key)
    if (index !== -1) {
      notifications.value.splice(index, 1)
    } 
  }

  const clear = () => {
    notifications.value = [] 
  }
  return {
    notifications,
    add, 
    remove,
    clear,
  }
})