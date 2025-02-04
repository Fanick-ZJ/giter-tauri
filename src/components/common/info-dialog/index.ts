// 使用组件，通过函数调用的方式，将组件附着在root上
import { ComponentPublicInstance, createVNode, nextTick, render } from 'vue'
import RepoInfoDialog from './index.vue'
import { RepoInfoProps } from './types'
import { Repository } from '@/types'
import { createSingletonComponent } from '@/utils/tool'

const className = '__repo_info_dialog__container'

export const useFileInfoDialog = (options: RepoInfoProps): (Promise<Repository>) => {
  const comp = createSingletonComponent({
    component: RepoInfoDialog,
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