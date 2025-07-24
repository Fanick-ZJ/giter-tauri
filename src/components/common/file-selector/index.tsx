// 使用组件，通过函数调用的方式，将组件附着在root上
import {Component, defineComponent, nextTick, ref, Ref, useTemplateRef, watch } from 'vue'
import FileSelector from './index.vue'
import { FileSelectorOptions } from './types'
import { createSingletonComponent } from '@/utils/tool'
import { useAbstractDialog, DialogOptions, DialogCallbacks } from '../abstract-dialog'
import FileTree from './tree.vue'
import { NInput } from 'naive-ui'
import { fileNameIconMap } from '../file-icon/fileicons'

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

export function useFileSelectorDialog(props: FileSelectorOptions) {
  let dialogActions: any;
  
  const options: DialogOptions = {
    containerName: className,
    buttonBox: 'ok-cancel',
    title: '选择文件',
  }

  const content = (): Component => {
    return defineComponent({
     name: 'FileSelectorDialog',
     setup () {
      const fileTreeRef = useTemplateRef<typeof FileTree>('fileTreeRef')
      const selected = ref('')
      const changed = (val: string) => {
        selected.value = val
        dialogActions.setReturnData(val)
      }
      watch(() => fileTreeRef.value?.checkedKeys, (val) => {
        if (val && val.length > 0) {
          selected.value = ''
          dialogActions.setReturnData(val)
        }
      })
       return () => (
        <div>
          <NInput placeholder='请输入文件路径' type='text' size='tiny' v-model:value={selected.value}/>
          <FileTree ref='fileTreeRef' {...props} onChange={changed}/>
        </div>
       )
     } 
    })
  }
  
  const callbacks: DialogCallbacks = {
    content
  }
  
  dialogActions = useAbstractDialog<string | string[]>(options, callbacks)
  
  return {
    ...dialogActions
  }
}

export function createFileSelectorDialog(props: FileSelectorOptions): Promise<string | string[]> {
  const dialog = useFileSelectorDialog(props)
  return dialog.showDialog()
}