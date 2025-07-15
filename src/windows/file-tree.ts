import { FileHistoryItem } from "@/types";
import { getRepoByPath } from "@/utils/command";
import { getRepositoryByPath } from "@/utils/store";
import { TauriEvent } from "@tauri-apps/api/event";
import { WebviewOptions } from "@tauri-apps/api/webview";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { WindowOptions, LogicalSize } from "@tauri-apps/api/window";

export const LOCAL_STORAGE_FIRST_FILE_HISTORY = "first-file-history"

class FileTreeWindow {
  private window: WebviewWindow | null = null;
  private repo = '';
  private commitId = '';

  constructor(repo: string, commitId: string) {
    this.repo = repo;
    this.commitId = commitId;
    this.buildWindow()

  }

  private buildWindow() {
    this.window = new WebviewWindow(`file-tree-${this.commitId}`, this.options())
    getRepoByPath(this.repo).then(repo => {
      this.window?.setTitle("提交树-" + repo.alias + " : " + this.commitId)
    })
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
      visible: false  // 新建时不显示窗口

    }
  }

  public async show() {
    if (!this.window) {
      this.window = new WebviewWindow("file-tree", this.options());
    }
    // 等待窗口真正创建完成
    await this.window.once(TauriEvent.WINDOW_CREATED, () => {})
    await this.window.show(); // 显式显示
    await this.window.setFocus();
  }
}

export default FileTreeWindow;