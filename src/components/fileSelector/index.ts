// 使用组件，通过函数调用的方式，将组件附着在root上
import { ComponentPublicInstance, createVNode, nextTick, render } from 'vue'
import FileSelector from './comp.vue'
import { FileSelectorOptions } from './types'

const className = '__file__selector__container'

let OPENED = false

const createInstance = (options: FileSelectorOptions): ComponentPublicInstance<typeof FileSelector> => {
  // 查找是否已经有存在的组件
  let container = document.querySelector(`.${className}`) as HTMLElement
  if (!container) {
    container = document.createElement('div')
    container.classList.add(className)
    document.body.appendChild(container)
  }
  // 创建虚拟节点
  const vm = createVNode(FileSelector, options)
  // 将虚拟节点挂载到容器上
  render(vm, container)
  return vm.component?.proxy as ComponentPublicInstance<typeof FileSelector>
}

export const unmount = () => {
  const container = document.querySelector(`.${className}`) as HTMLElement
  if (container) {
    render(null, container)
    document.body.removeChild(container)
  }
}

export const useFileSelector = (options: FileSelectorOptions): (Promise<Array<string>>) => {
  if (OPENED) {
    throw new Error('已经有一个文件选择器在运行')
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