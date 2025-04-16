import { WindowsLabel } from "@/const/windows"
import { Window, WindowOptions } from "@tauri-apps/api/window"

export const hasWindow = async (label: WindowsLabel) => {
  return Window.getByLabel(label).then((window) => {
    return !!window
  }) 
}

export const getWindow = async (label: WindowsLabel, options?: WindowOptions) => {
  return Window.getByLabel(label).then((window) => {
    if (window) {
      return window 
    } else {
     return new Window(label, options) 
    }
  })
}

export const sendMessage = async (label: WindowsLabel, message: string, payload?: unknown) => {
  Window.getByLabel(label).then((window) => {
    if (window) {
      window.emit(message, payload)
    }
  })
}