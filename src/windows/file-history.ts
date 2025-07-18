import { FileHistoryItem } from "@/types";
import { TauriEvent } from "@tauri-apps/api/event";
import { WebviewOptions } from "@tauri-apps/api/webview";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { WindowOptions, Window, LogicalSize } from "@tauri-apps/api/window";

export const LOCAL_STORAGE_FIRST_FILE_HISTORY = "first-file-history"

export type FileHistory = {
  repo: string,
  path: string,
  history: FileHistoryItem[]
}

class FileHistoryWindow {
  static FILE_ADD = "file-add"
  private static window: WebviewWindow;
  private static hasWindow = false;

  private static buildWindow() { 
    FileHistoryWindow.window = new WebviewWindow("file-history", FileHistoryWindow.options())
    FileHistoryWindow.window.once(TauriEvent.WINDOW_CREATED, () => {
      FileHistoryWindow.window.setMinSize(new LogicalSize(800, 600))
    })
    FileHistoryWindow.hasWindow = true;
    FileHistoryWindow.window.once(TauriEvent.WINDOW_DESTROYED, () => {
      FileHistoryWindow.hasWindow = false 
    })
  }

  private static options(): Omit<WebviewOptions, 'x' | 'y' | 'width' | 'height'> & WindowOptions {
    return {
      title: "文件历史",
      width: 800,
      height: 600,
      resizable: true,
      maximizable: true,
      fullscreen: false,
      url: '/file_history',
      parent: 'main',
      center: true

    }
  }

  public static show() {
    FileHistoryWindow.clearHistory()
    if (FileHistoryWindow.hasWindow) { 
      FileHistoryWindow.window.setFocus()
    } else {
      FileHistoryWindow.buildWindow()
    }
    return FileHistoryWindow.window
  }

  public static addHistoryTab(repo: string, history: FileHistoryItem[], focusCommit: string | undefined = undefined) {

    const path = history[0].file.path
    if (!FileHistoryWindow.hasWindow) {
      // 将初次打开时携带的参数，放在localStorage中，方便后续使用
      localStorage.setItem(LOCAL_STORAGE_FIRST_FILE_HISTORY, JSON.stringify({ repo, path, history, focusCommit }))
      FileHistoryWindow.buildWindow()
    } else {
      FileHistoryWindow.show()
      FileHistoryWindow.window.emit(FileHistoryWindow.FILE_ADD, { repo, path, history, focusCommit })   
    } 
  }

  public static clearHistory() {
    localStorage.removeItem(LOCAL_STORAGE_FIRST_FILE_HISTORY) 
  }
}

export default FileHistoryWindow;