// 使用组件，通过函数调用的方式，将组件附着在root上
import { ComponentPublicInstance, createVNode, nextTick, render } from 'vue'
import RepoInfoDialog from './index.vue'
import { RepoInfoProps } from './types'
import { Repository } from '@/types/store'

const className = '__repo_info_dialog__container'

let OPENED = false

const createInstance = (options: RepoInfoProps): ComponentPublicInstance<typeof RepoInfoDialog> => {
  // 查找是否已经有存在的组件
  let container = document.querySelector(`.${className}`) as HTMLElement
  if (!container) {
    container = document.createElement('div')
    container.classList.add(className)
    document.body.appendChild(container)
  }
  // 创建虚拟节点
  const vm = createVNode(RepoInfoDialog, options)
  // 将虚拟节点挂载到容器上
  render(vm, container)
  return vm.component?.proxy as ComponentPublicInstance<typeof RepoInfoDialog>
}

export const unmount = () => {
  const container = document.querySelector(`.${className}`) as HTMLElement
  if (container) {
    render(null, container)
    document.body.removeChild(container)
  }
}

export const useFileInfoDialog = (options: RepoInfoProps): (Promise<Repository>) => {
  if (OPENED) {
    throw new Error('已经有一个仓库设置对话框在运行')
  }
  const comp = createInstance(options)
  OPENED = true
  // 监听关闭事件
  comp.$.exposed?.closeCb( async () => {
    await nextTick()
    unmount()
    OPENED = false
  })
  return comp.$.exposed?.show()
}