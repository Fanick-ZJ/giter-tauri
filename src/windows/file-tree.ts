import { FileHistoryItem } from "@/types";
import { TauriEvent } from "@tauri-apps/api/event";
import { WebviewOptions } from "@tauri-apps/api/webview";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { WindowOptions, Window, LogicalSize } from "@tauri-apps/api/window";

export const LOCAL_STORAGE_FIRST_FILE_HISTORY = "first-file-history"

class FileTreeWindow {
  private static openedRepo = new Map<string, FileTreeWindow>()
  private window: WebviewWindow | null = null;
  private repo = '';
  private commitId = '';

  constructor(repo: string, commitId: string) {
    if (FileTreeWindow.openedRepo.has(repo)) {
      return FileTreeWindow.openedRepo.get(repo) as FileTreeWindow; 
    }
    this.repo = repo;
    this.commitId = commitId;
    this.buildWindow()
    FileTreeWindow.openedRepo.set(repo, this)
    this.window?.once(TauriEvent.WINDOW_DESTROYED, () => {
      FileTreeWindow.openedRepo.delete(repo)
    })

  }

  private buildWindow() { 
    this.window = new WebviewWindow("file-tree", this.options())
    this.window.once(TauriEvent.WINDOW_CREATED, () => {
        this.window && this.window.setMinSize(new LogicalSize(800, 600))
        this.window && this.window.center()
    })
  }

  private options(): Omit<WebviewOptions, 'x' | 'y' | 'width' | 'height'> & WindowOptions {
    return {
      title: "提交文件树",
      width: 800,
      height: 600,
      resizable: true,
      maximizable: true,
      fullscreen: false,
      url: `/file_tree/${encodeURIComponent(this.repo)}/${this.commitId}`,
      parent: 'main',
      center: true,

    }
  }

  public async show() {
    await this.window?.setFocus()
  }
}

export default FileTreeWindow;