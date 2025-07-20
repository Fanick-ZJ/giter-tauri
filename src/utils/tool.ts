import { SEPERATOR } from "@/const"
import { useThemeStore } from "@/store/modules/theme";
import { openPath } from '@tauri-apps/plugin-opener';
import _ from "lodash";
import { darkTheme, NConfigProvider, NDialogProvider } from "naive-ui";
import { Component, ComponentPublicInstance, computed, createVNode, render, VNode, VNodeProps, watchEffect } from "vue";

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

  // 创建容器
  let container = document.querySelector(`.${className}`) as HTMLElement
  
  if (!container) {
    container = document.createElement('div')
    container.className = className
    parent.appendChild(container)
  }

  let vm: VNode
  let stopWatcher: (() => void) | null = null

  // 创建响应式渲染函数
  const renderWithTheme = () => {
    vm = createVNode(component, options.props)
    // 获取主题配置
    const themeStore = useThemeStore()
    const dialogProvider = createVNode(NConfigProvider, { 
      theme: themeStore.isDark ? darkTheme : undefined 
    }, {
      default: () => [
        createVNode(NDialogProvider, null, {
          default: () => [vm]
        })
      ]
    })
    render(dialogProvider, container)
    return vm
  }

  // 初始渲染并监听主题变化
  vm = renderWithTheme()
  stopWatcher = watchEffect(() => {
    renderWithTheme()
  })

  // 创建卸载方法
  const unmount = () => {
    if (stopWatcher) {
      stopWatcher()
    }
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
    // ── JavaScript 系 ──
    ts: 'typescript',
    js: 'javascript',
    jsx: 'javascriptreact',
    tsx: 'typescriptreact',
    mjs: 'javascript',          // Node ES Module
    cjs: 'javascript',          // Node CommonJS

    // ── 样式表 ──
    css: 'css',
    scss: 'scss',
    sass: 'sass',
    less: 'less',
    stylus: 'stylus',

    // ── HTML / 模板 ──
    html: 'html',
    htm: 'html',
    svelte: 'svelte',
    vue: 'vue',
    astro: 'astro',
    jinja: 'jinja',
    jinja2: 'jinja',
    njk: 'nunjucks',
    hbs: 'handlebars',
    mustache: 'handlebars',
    ejs: 'ejs',
    pug: 'pug',
    twig: 'twig',

    // ── JSON / 数据 ──
    json: 'json',
    jsonc: 'jsonc',             // 带注释的 JSON
    json5: 'json5',
    toml: 'toml',
    yaml: 'yaml',
    yml: 'yaml',

    // ── 脚本 / Shell ──
    sh: 'shell',
    bash: 'shell',
    zsh: 'shell',
    fish: 'fish',
    ps1: 'powershell',
    pwsh: 'powershell',

    // ── 后端 / 系统 ──
    py: 'python',
    pyi: 'python',
    java: 'java',
    kt: 'kotlin',
    kts: 'kotlin',
    scala: 'scala',
    groovy: 'groovy',
    gradle: 'groovy',           // build.gradle
    cs: 'csharp',
    fs: 'fsharp',
    vb: 'vb',
    c: 'c',
    h: 'c',
    cc: 'cpp',
    cpp: 'cpp',
    cxx: 'cpp',
    hpp: 'cpp',
    mm: 'objective-c',
    m: 'objective-c',
    swift: 'swift',
    rs: 'rust',
    go: 'go',
    php: 'php',
    rb: 'ruby',
    r: 'r',
    dart: 'dart',
    clj: 'clojure',
    cljs: 'clojure',
    elm: 'elm',
    erl: 'erlang',
    ex: 'elixir',
    exs: 'elixir',
    hs: 'haskell',
    lhs: 'haskell',
    nim: 'nim',
    zig: 'zig',
    v: 'v',

    // ── 数据库 / 查询 ──
    sql: 'sql',
    mysql: 'sql',
    pgsql: 'sql',
    psql: 'sql',
    sqlite: 'sql',

    // ── 标记 / 文档 ──
    md: 'markdown',
    markdown: 'markdown',
    rst: 'restructuredtext',
    tex: 'latex',
    latex: 'latex',
    bib: 'bibtex',
    adoc: 'asciidoc',
    org: 'org',

    // ── 配置 / DevOps ──
    dockerfile: 'dockerfile',
    nginx: 'nginx',
    conf: 'properties',         // 通用 conf
    ini: 'ini',
    cfg: 'ini',
    env: 'properties',
    envrc: 'shell',
    gitignore: 'gitignore',
    gitattributes: 'gitattributes',
    gitmodules: 'gitconfig',
    prettierrc: 'json',
    prettierrc_js: 'javascript',
    prettierrc_ts: 'typescript',
    eslintrc: 'json',
    eslintrc_js: 'javascript',
    eslintrc_ts: 'typescript',
    babelrc: 'json',
    babelrc_js: 'javascript',
    stylelintrc: 'json',
    stylelintrc_js: 'javascript',
    svgo_yml: 'yaml',
    prettierignore: 'ignore',
    eslintignore: 'ignore',
    npmignore: 'ignore',
    dockerignore: 'ignore',

    // ── 纯文本 / 日志 / 其他 ──
    txt: 'plaintext',
    log: 'log',
    csv: 'csv',
    tsv: 'tsv',
    diff: 'diff',
    patch: 'diff',
    http: 'http',
    graphql: 'graphql',
    proto: 'proto3',            // Protocol Buffer
    reg: 'reg',
    wasm: 'wasm',
    asm: 'asm',
    vbs: 'vbscript',
    bat: 'batch',
    cmd: 'batch',
    makefile: 'makefile',
    mk: 'makefile',
    cmake: 'cmake',
    prisma: 'prisma',
    graphqls: 'graphql',
    mdx: 'mdx',
    svx: 'markdown',            // Svelte MDX
    rmd: 'r',                   // R Markdown
    q: 'q',                     // kdb+/q
    k: 'k',                     // kdb+/q
    rego: 'rego',               // Open Policy Agent
    cue: 'cue',
    cue_mod: 'cue',
    cue_sum: 'cue',
  };
  // @ts-ignore
  return languageMapping[fileExtension]  || 'plaintext'
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
  if (idx === -1) return undefined
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

export function isTextOverflow(text: string, container: HTMLElement): boolean {
  const canvas = document.createElement('canvas')
  const context = canvas.getContext('2d')!
  // 获取容器实际样式
  const style = window.getComputedStyle(container)
  
  // 设置测量上下文样式（需要与容器实际样式一致）
  context.font = `${style.fontWeight} ${style.fontSize} ${style.fontFamily}`
  
  // 计算文本测量宽度
  const textWidth = context.measureText(text).width
  // 获取容器内容宽度（需要减去 padding）
  const containerWidth = container.clientWidth 
    - parseFloat(style.paddingLeft)
    - parseFloat(style.paddingRight)

  canvas.remove()
  return textWidth > containerWidth
}