<script setup lang="ts">
import { useElementSize } from '@vueuse/core';
import * as monaco from 'monaco-editor';
import { onMounted, ref, watch } from 'vue';

const props = defineProps({
  language: {
    type: String,
    default: 'plain-text'
  },
  content: {
    type: String,
    default: ''
  },
  readonly: {
    type: Boolean,
    default: false
  }
})

let editor: monaco.editor.IStandaloneCodeEditor;
const editorBody = ref<HTMLElement>();
const initEditor = () => {
  if (!editorBody.value) return

  editor = monaco.editor.create(editorBody.value, {
    value: props.content,
    language: props.language,
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
  editor.getDomNode()!.style.width = '100%'
  editor.getDomNode()!.style.height = '100%' // 新增高度设置
  updateEditorHeight()
}

// 更新编辑器高度
const updateEditorHeight = () => {
  if (!editor) return;
  const height = calculateEditorHeight();
  editor.getDomNode()!.style.height = `${height}px`; // 设置高度
  editor.layout(); // 重新布局
}

function calculateEditorHeight() {
  const lineHeight = editor.getOption(monaco.editor.EditorOption.lineHeight);
  const lineCount = editor.getModel()!.getLineCount();
  const scrollBeyondLastLine = editor.getOption(monaco.editor.EditorOption.scrollBeyondLastLine);
  
  // 计算基础高度（所有行高之和）
  let totalHeight = lineHeight * lineCount;
  
  // 添加滚动边界的额外空间（每超出1行增加1行高度）
  if (scrollBeyondLastLine) {
    totalHeight += lineHeight;
  }
  
  // 获取容器可用高度
  const containerHeight = editorBody.value?.parentElement?.clientHeight || 0;
  
  // 返回两者中的较小值，确保不超过容器高度
  return Math.min(totalHeight, containerHeight) - 50;
}
  
onMounted(() => {
  initEditor()
})
</script>

<template>
  
  <div class="editor-container w-full h-full flex flex-col">
    <div class="editor-header">
      <slot name="header"/>
    </div>
    <div class="flex-1 min-h-0 relative">
      <div class="editor-body absolute w-full h-full flex-auto" ref="editorBody"></div>
    </div>
  </div>
</template>