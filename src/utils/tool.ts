import { SEPERATOR } from "@/const"
import { openPath } from '@tauri-apps/plugin-opener';
import _ from "lodash";
import { NDialogProvider } from "naive-ui";
import { Component, ComponentPublicInstance, createVNode, render, VNode, VNodeProps } from "vue";

// 打开指定路径的文件管理器
export async function openFileManager(path: string) {
  try {
    await openPath(path);
    console.log('文件管理器已打开');
  } catch (error) {
    console.error('打开文件管理器失败:', error);
  }
}
export const getDirName = (path: string) => {
  return path.split(SEPERATOR).pop()
}

export type InstanceProps = (Record<string, unknown> & VNodeProps);

type CreateInstanceOptions<T extends Component> = {
  component: T // 要挂载的组件
  props?: (Record<string, unknown> & VNodeProps) | null
  className: string    // 容器唯一类名
  parent?: HTMLElement // 父容器 (默认document.body)
}

export type InstanceManager<T extends Component> = {
  instance: ComponentPublicInstance<T>
  unmount: () => void
}

const instanceMap = new Map<string, InstanceManager<any>>()

// 创建单例组件
export function createSingletonComponent<T extends Component>(
  options: CreateInstanceOptions<T>
): InstanceManager<T> {
  const {
    component,
    className,
    parent = document.body
  } = options

  // 检查已有实例
  const existing = instanceMap.get(className)
  if (existing) {
    throw new Error(`Component with class name ${className} already exists`)
  }

  // 创建容器，这里还需要为容器添加额外的NDialogProvider之类的组件
  let container = document.querySelector(`.${className}`) as HTMLElement
  
  if (!container) {
    container = document.createElement('div')
    container.className = className
    parent.appendChild(container)
  }
  // 创建虚拟节点
  const vm = createVNode(component, options.props)
  const dialogProvider = createVNode(NDialogProvider, null, {
    default: () => [vm]
  })

  // 挂载组件
  render(dialogProvider, container)

  // 创建卸载方法
  const unmount = () => {
    render(null, container)
    container.remove()
    instanceMap.delete(className)
  }

  // 保存实例
  const instanceManager = {
    instance: vm.component?.proxy as ComponentPublicInstance<T>,
    unmount
  }

  instanceMap.set(className, instanceManager)

  return instanceManager
}

export const fileExtension = (path: string) => {
  return path.split('.').pop() || '' 
}

export function getMonacoLanguage(fileName: string) {
  const fileExtension = fileName.split('.').pop()?.toLowerCase() || '';

  const languageMapping = {
    ts: 'typescript',
    js: 'javascript',
    jsx: 'javascriptreact',
    tsx: 'typescriptreact',
    html: 'html',
    css: 'css',
    scss: 'scss',
    less: 'less',
    json: 'json',
    md: 'markdown',
    py: 'python',
    java: 'java',
    c: 'c',
    cpp: 'cpp',
    cs: 'csharp',
    php: 'php',
    rb: 'ruby',
    go: 'go',
    sh: 'shell',
    sql: 'sql',
    xml: 'xml',
    yml: 'yaml',
    yaml: 'yaml',
    dockerfile: 'dockerfile',
    gitignore: 'plaintext',
    txt: 'plaintext',
    log: 'plaintext',
  };

  // @ts-ignore
  return languageMapping[fileExtension] || 'plaintext';
}

export const getWeekNumber = (date: Date) => {
  var target  = new Date(date.valueOf());
  var dayNr   = (date.getDay() + 6) % 7;
  target.setDate(target.getDate() - dayNr + 3);
  var firstThursday = target.valueOf();
  target.setMonth(0, 1);
  if (target.getDay() != 4) {
      target.setMonth(0, 1 + ((4 - target.getDay()) + 7) % 7);
  }
  return Math.ceil((firstThursday - target.getTime()) / 604800000);
}

/**
 * 获取每个月的天数
 * @param date 
 */
export const getDaysOfMonth = (year: number) => {
  const isLeapYear = (year % 4 === 0 && year % 100 !== 0) || year % 400 === 0;
  const daysInMonth = [31, isLeapYear ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  return daysInMonth;
}

export const basename = (path: string) => {
  let idx = path.lastIndexOf('/') 
  idx = idx > -1 ? idx : path.lastIndexOf('\\')
  return path.slice(idx + 1)
}

export const extname = (path: string) => {
  const idx = path.lastIndexOf('.')
  if (idx === -1) return ''
  return path.slice(idx + 1) 
}

export async function withMinDelay<T extends (...args: any[]) => Promise<any>>(
  fn: T,
  delay: number,
  cb?: () => void
): Promise<Awaited<ReturnType<T>>>{
  const start = Date.now()
  const res = await fn()
  const elapsed = Date.now() - start
  const remaining = Math.max(delay - elapsed, 0)
  await new Promise(resolve => setTimeout(resolve, remaining))
  cb && cb()
  return res
}

export const bytesToString = (buffer: number[]) => {
  const view = new Uint8Array(buffer);
  const decoder = new TextDecoder('utf-8');
  const str = decoder.decode(view)
  return str;
}