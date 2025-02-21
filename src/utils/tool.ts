import { SEPERATOR } from "@/const"
import { open as openShell } from '@tauri-apps/plugin-shell';
import { Component, ComponentPublicInstance, createVNode, render, VNode, VNodeProps } from "vue";

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

type Data = Record<string, unknown>;

type CreateInstanceOptions<T extends Component> = {
  component: T // 要挂载的组件
  props?: (Data & VNodeProps) | null
  className: string    // 容器唯一类名
  parent?: HTMLElement // 父容器 (默认document.body)
}

type InstanceManager<T extends Component> = {
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

  // 创建容器
  let container = document.querySelector(`.${className}`) as HTMLElement
  if (!container) {
    container = document.createElement('div')
    container.className = className
    parent.appendChild(container)
  }

  // 创建虚拟节点
  const vm = createVNode(component, options.props)

  // 挂载组件
  render(vm, container)

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