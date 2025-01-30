import { SEPERATOR } from "@/const"
import { open as openShell } from '@tauri-apps/plugin-shell';

// 打开指定路径的文件管理器
export async function openFileManager(path: string) {
  try {
    await openShell(path);
    console.log('文件管理器已打开');
  } catch (error) {
    console.error('打开文件管理器失败:', error);
  }
}
export const getDirName = (path: string) => {
  return path.split(SEPERATOR).pop()
}