export enum NotificationLevel {
  Info = 'info',
  Success = 'success',
  Warning = 'warning', 
  Error = 'error',
}

export const NotificationType =  {
  Owner: 'owner',
  Other: 'other',
} as const;


type KeyType = `${typeof NotificationType[keyof typeof NotificationType]}:${string}`

// 消息回调,返回boolean表示是否关闭消息
export type MessageCb = {
  name: string,
  cb: (...args: any[]) => Promise<boolean | void> | boolean | undefined,
}

export type NotificationBody = {
  title: string,
  body: string | (() => import("vue").VNodeChild),
  type: NotificationLevel,
  duration?: number,
  cb?: MessageCb[] | MessageCb,
  readonly key: KeyType,
}