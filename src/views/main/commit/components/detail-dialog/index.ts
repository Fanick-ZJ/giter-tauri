// 使用组件，通过函数调用的方式，将组件附着在root上
import CommitDetailComponent from './index.vue'
import { CommitDetailProps } from './types'
import { createSingletonComponent } from '@/utils/tool'

const className = '__commti_detail__container'

export const useCommitDetailDialog = (options: CommitDetailProps) => {
  const comp = createSingletonComponent({
    component: CommitDetailComponent,
    props: options,
    className
  })

  comp.instance.$.exposed?.setUnmount(() => {
    console.log('组件卸载')
    comp.unmount()
  })
  return comp.instance.$.exposed?.show()
}