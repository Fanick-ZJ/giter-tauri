// 使用组件，通过函数调用的方式，将组件附着在root上
import {Component, defineComponent, nextTick, ref, Ref, useTemplateRef, watch } from 'vue'
import FileSelector from './index.vue'
import { FileSelectorOptions } from './types'
import { createSingletonComponent } from '@/utils/tool'
import { AbstractDialog } from '../abstract-dialog'
import FileTree from './tree.vue'
import { NInput } from 'naive-ui'

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

export class FileSelectorDialog extends AbstractDialog<String | String[]> {
  private props: FileSelectorOptions
  constructor (props: FileSelectorOptions) {
    super({
      containerName: className,
      buttonBox: 'ok-cancel',
      title: '选择文件',
    }) 
    this.props = props
  }

  public content(): Component {
    const _this = this
    return defineComponent({
     name: 'FileSelectorDialog',
     setup () {
      const fileTreeRef = useTemplateRef<typeof FileTree>('fileTreeRef')
      const selected = ref('')
      const changed = (val: string) => {
        selected.value = val
        _this.setReturnData(val)
      }
      watch(() => fileTreeRef.value?.checkedKeys, (val) => {
        if (val && val.length > 0) {
          selected.value = ''
          _this.setReturnData(val)
        }
      })
       return () => (
        <div>
          <NInput placeholder='请输入文件路径' type='text' size='tiny' v-model:value={selected.value}/>
          <FileTree ref='fileTreeRef' {..._this.props} onChange={changed}/>
        </div>
       )
     } 
    })
  }
}