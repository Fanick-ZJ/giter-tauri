<script setup lang="ts">
import { useThemeStore } from '@/store/modules/theme'
import { extname, getMonacoLanguage } from '@/utils/tool'
import * as monaco from 'monaco-editor'
import { getCurrentInstance, onMounted, ref, watch } from 'vue'
import { FileOption } from './types';

const props = defineProps<{
    original: FileOption,
    modified: FileOption,
    readonly: boolean
}>()

const instance = getCurrentInstance()
let editor: monaco.editor.IStandaloneDiffEditor;
const editorBody = ref<HTMLElement>();
const themeStore = useThemeStore()


const autoDetectLanguage = (type: 'original' | 'modified') => {
  const has_language = instance?.vnode.props?.[type]['language'] != undefined
  if (has_language) {
    return instance?.vnode.props?.[type]['language']
  }
  return getMonacoLanguage(instance?.vnode.props?.[type]['filename'] || '') 
}

const useEditorModel = () => {
    const originModel = monaco.editor.createModel(props.original.content, autoDetectLanguage('original'))
    const modifiedModel = monaco.editor.createModel(props.modified.content, autoDetectLanguage('modified'))
    
    watch(() => props.original.content, (newVal, oldVal) => {
        originModel.setValue(newVal!)
    })

    watch(() => props.modified.content, (newVal, oldVal) => {
        modifiedModel.setValue(newVal!)
    })

    return {
        originModel,
        modifiedModel
    }
}

const { originModel, modifiedModel} = useEditorModel()

// 监听主题变化
watch(() => themeStore.isDark, (isDark) => {
  if (editor) {
    monaco.editor.setTheme(isDark ? 'vs-dark' : 'vs')
  }
})

const initEditor = () => {
  if (!editorBody.value) return

  editor = monaco.editor.createDiffEditor(editorBody.value, {
    theme: themeStore.isDark ? 'vs-dark' : 'vs', // 根据主题设置
    minimap: {
      enabled: false,
    },
    scrollBeyondLastLine: false, // 禁用在最后一行之后滚动
    automaticLayout: true, // 自动布局
    scrollbar: {
      // vertical: 'hidden', // 禁用纵向滚动条
      horizontal: 'auto', // 保持横向滚动条自动显示
      handleMouseWheel: true, // 监听鼠标滚轮事件
      alwaysConsumeMouseWheel: false, // 允许滚动事件冒泡
    },
    readOnly: props.readonly, // 设置只读模式
    contextmenu: false,
  })
}
  
onMounted(() => {
  initEditor()
  editor.setModel({
    original: originModel,
    modified: modifiedModel
  })
})
</script>

<template>
  
  <div class="editor-container w-full h-full flex flex-col">
    <div class="editor-header">
      <slot name="header"/>
    </div>
    <div class="flex-1 relative">
      <div class="editor-body absolute w-full h-full flex-auto" ref="editorBody"></div>
    </div>
  </div>
</template>