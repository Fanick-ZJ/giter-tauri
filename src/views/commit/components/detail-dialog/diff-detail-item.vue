<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, onUnmounted, PropType, Ref, ref } from 'vue';
import { DiffContent, File } from '@/types';
import { Icon } from '@iconify/vue';
import { NCard } from 'naive-ui';
import * as monaco from 'monaco-editor';
import { getMonacoLanguage } from '@/utils/tool';
import { fileDiff, getBlobContent } from '@/utils/command';

defineOptions({
  name: 'DiffDetailComponent' 
})

const props = defineProps({
  repo: {
    type: String,
    required: true 
  },
  file: {
    type: Object as PropType<File>,
    required: true
  }
})

const diffContent = ref<DiffContent>()
let addedLines: Ref<number[]> = ref([])
let deletedLines: Ref<number[]> = ref([])
let diffDetailLines: Ref<number[]> = ref([])
const success = ref<Boolean>(false)
const loading = ref<Boolean>(true)

// 暴露给外部调用，动态加载，避免拥堵
const load = async () => {
  if (props.file.isBinary) {
    success.value = false
    loading.value = false
    return
  }
  if (props.file.status === 'Added') {
    getBlobContent(props.repo, props.file.objectId).then(async res => {
      // 将u8数组转换为字符串
      const view = new Uint8Array(res);
      for (let i = 0; i < res.length; i++) {
        view[i] = res[i];
      }
      const decoder = new TextDecoder('utf-8');
      const str = decoder.decode(view)
      success.value = true
      addedLines.value = str.split('\n').map((_, i) => i)
      
      await nextTick()
      initEditor(str)
      applyEditorStyle()
      loading.value = false
    })
  }
  else if (props.file.status === 'Deleted') {
    getBlobContent(props.repo, props.file.prevObjectId).then(async res => {
      // 将u8数组转换为字符串
      const view = new Uint8Array(res);
      const decoder = new TextDecoder('utf-8');
      const str = decoder.decode(view)
      success.value = true
      deletedLines.value = str.split('\n').map((_, i) => i)

      await nextTick()
      initEditor(str)
      applyEditorStyle()
    }).catch(err => {
      console.error(err)
      success.value = false
    }).finally(() => {
      loading.value = false
    })
  }
  else {
    fileDiff(props.repo, props.file.prevObjectId, props.file.objectId).then(async res => {
      diffDetailLines.value = findDiffInfoLine(res.display)
      const lines = splitModifLines(res.display)
      addedLines.value = lines.added
      deletedLines.value = lines.deleted
      res.display = lines.content
      diffContent.value = res
      success.value = true
      await nextTick()
      initEditor(diffContent.value.display)
      applyEditorStyle()
    }).catch(err => {
      console.error(err)
      success.value = false
    }).finally(() => {
      loading.value = false
    })
  }	
}


const modifRatiStyle = computed(() => {
  const a = addedLines.value.length / (addedLines.value.length + deletedLines.value.length)
  return {
    background: `linear-gradient(to right, #4ade80 ${a * 100}%, #f87171 ${a * 100}%)`,
  }
})

defineExpose({
  load
})

const editorContainer = ref<HTMLElement>();
let editor: monaco.editor.IStandaloneCodeEditor;

function calculateEditorHeight() {
  const lineHeight = editor.getOption(monaco.editor.EditorOption.lineHeight);
  const lineCount = editor.getModel()!.getLineCount();
  const scrollBeyondLastLine = editor.getOption(monaco.editor.EditorOption.scrollBeyondLastLine);

  // 如果滚动条超出最后一行，调整高度
  if (scrollBeyondLastLine) {
    const scrollBarHeight = lineHeight; // 假设滚动条高度为行高
    return lineHeight * lineCount - scrollBarHeight;
  }

  return lineHeight * lineCount;
}

// 更新编辑器高度
const updateEditorHeight = () => {
  if (!editor) return;
  const height = calculateEditorHeight();
  editor.getDomNode()!.style.height = `${height}px`; // 设置高度
  editor.layout(); // 重新布局
}

onBeforeUnmount(() => {
  editor && editor.dispose();	
})

const initEditor = (content: string) => {
  if (!editorContainer.value) return;
  editor = monaco.editor.create(editorContainer.value, {
    value: content,
    language: getMonacoLanguage(props.file.path),
    minimap: {
      enabled: false,
    },
    scrollBeyondLastLine: false, // 禁用在最后一行之后滚动
    automaticLayout: true, // 自动布局
  scrollbar: {
    vertical: 'hidden', // 禁用纵向滚动条
    horizontal: 'auto', // 保持横向滚动条自动显示
    handleMouseWheel: true, // 监听鼠标滚轮事件
    alwaysConsumeMouseWheel: false, // 允许滚动事件冒泡
  },
  readOnly: true,
	contextmenu: false,
  lineNumbers: 'off'
  });
  updateEditorHeight()
};

// 找到diff信息行
const findDiffInfoLine = (content: String) => {
  const reg = /^@@ -(\d*),(\d*) \+(\d*),(\d*) @@$/
  const n: number[] = []
  const lines = content.split('\n')
  for (let i = 0 ; i < lines.length; i++) {
    if (lines[i].search(reg) >= 0) {
      n.push(i)
    }
  }
  return n
}

// 找到修改行和去除修改行的符号
const splitModifLines = (content: String) => {
  const added: number[] = []
  const deleted: number[] = []
  const c = content.split('\n').map((line, index) => {
    if (line[0] == '+' || line[0] == '-') {
      if (line[0] == '+') {
        added.push(index) 
      }
      if (line[0] == '-') {
        deleted.push(index) 
      }
      return line.slice(1)
    }
    return line.slice(1)
  }).join('\n')
  return {
    added,
    deleted,
    content: c
  }
}
const applyEditorStyle = () => {
  const addedDecorations: monaco.editor.IModelDeltaDecoration[] = addedLines.value.map((lineNumber: number) => {
    return {
      range: new monaco.Range(lineNumber + 1, 1, lineNumber + 1, 1),
      options: {
        isWholeLine: true,
        className: 'added-line',
      }
    }
  });
  const deletedDecorations: monaco.editor.IModelDeltaDecoration[] = deletedLines.value.map((lineNumber: number) => {
    return {
      range: new monaco.Range(lineNumber + 1, 1, lineNumber + 1, 1),
      options: {
        isWholeLine: true,
        className: 'deleted-line', 
      } 
    }
  })
  const diffDetailDecorations: monaco.editor.IModelDeltaDecoration[] = diffDetailLines.value.map((lineNumber: number) => {
    return {
      range: new monaco.Range(lineNumber + 1, 1, lineNumber + 1, 1),
      options: {
        isWholeLine: true,
        className: 'diff-detail-line',
      } 
    } 
  })
  const decorations = editor.createDecorationsCollection([...addedDecorations,
                                                          ...deletedDecorations, 
                                                          ...diffDetailDecorations]);
  return decorations
}
</script>

<template>
  <NCard :header-style="{ position: 'sticky', top: '-1px', background: 'white', zIndex: 3 }">
    <template #header>
      <div>
        {{ file.path }}
      </div>
    </template>
    <template #header-extra>
      <div class="flex gap-1 items-center">
        <div class="text-green-400">
          {{ + addedLines.length  }}
        </div>
        <div class="text-red-400">
          {{ + deletedLines.length  }}
        </div>
        <div class="h-[10px] w-[30px]" :style="modifRatiStyle"></div>
      </div>
    </template>
    <div v-if="loading" class="w-full h-full flex justify-center items-center">
      <Icon icon="eos-icons:bubble-loading" width="24" height="24" />
    </div>
    <div v-if="success" ref="editorContainer"></div>
    <div v-else-if="props.file.isBinary">
      二进制文件
    </div>
    <div v-else-if="!loading && !success">
      加载失败
    </div>
  </NCard>
</template>


<style scoped>
</style>