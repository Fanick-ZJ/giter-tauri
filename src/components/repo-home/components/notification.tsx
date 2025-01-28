import { NotificationBody } from "@/store/modules/notification/type";
import { NButton, NFlex } from 'naive-ui'

export const createNofication = (info: NotificationBody) => {
  let n = window.$notification.create({
    title: info.title,
    content: info.body,
    type: info.type,
    duration: info.duration,
    action: () => {
      if (!info.cb) return null
      const cbs = Array.isArray(info.cb) ? info.cb : [info.cb]
      const buttons = cbs.map((cb) => {
        return <NButton onClick={() => {
                let result = cb.cb()
                if (result instanceof Promise) {
                  result.then((res) => {
                    res && n.destroy()
                  })
                } else if (result === true) {
                  n.destroy()
                }
                }}
              >
                {cb.name}
              </NButton> 
      })
      if (buttons) {
        return <>
        <NFlex>
          {buttons}
        </NFlex>
      </> 
      }
      return null
    }
  })
}