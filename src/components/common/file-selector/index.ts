// 使用组件，通过函数调用的方式，将组件附着在root上
import {nextTick } from 'vue'
import FileSelector from './index.vue'
import { FileSelectorOptions } from './types'
import { createSingletonComponent } from '@/utils/tool'

const className = '__file__selector__container'

export const useFileSelector = (options: FileSelectorOptions): (Promise<string>) => {
  const comp = createSingletonComponent({
    component: FileSelector,
    props: options,
    className
  })
  // 监听关闭事件
  comp.instance.$.exposed?.closeCb( async () => {
    await nextTick()
    comp.unmount()
  })
  return comp.instance.$.exposed?.show()
}