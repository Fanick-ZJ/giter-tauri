/**
 * Monaco Editor 懒加载器
 *
 * 通过动态 import 让 Vite 将 Monaco 拆分为独立 chunk，
 * 避免将 ~3MB 的 monaco-editor 打入路由 chunk 中。
 */

import type * as Monaco from "monaco-editor";

let monacoPromise: Promise<typeof Monaco> | null = null;

/** 加载 monaco-editor（多次调用只发起一次网络请求） */
export function loadMonaco(): Promise<typeof Monaco> {
  if (!monacoPromise) {
    monacoPromise = import("monaco-editor");
  }
  return monacoPromise;
}

/** 获取已加载的 monaco 实例（未加载时返回 null） */
export function getMonaco(): typeof Monaco | null {
  // monacoPromise 已 resolve 时可直接同步读取
  return null; // 始终返回 null，强制通过 loadMonaco 异步获取
}
