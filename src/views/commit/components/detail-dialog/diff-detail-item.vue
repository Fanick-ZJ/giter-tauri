<script setup lang="ts">
import { computed, nextTick, onMounted, PropType, ref } from 'vue';
import { DiffContent, File } from '@/types';
import { NCard } from 'naive-ui';
import * as monaco from 'monaco-editor';
import { getMonacoLanguage } from '@/utils/tool';
import { fileDiff } from '@/utils/command';

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
let addedLines: number[] = []
let deletedLines: number[] = []
let diffDetailLines: number[] = []
let decorations: monaco.editor.IEditorDecorationsCollection
const success = ref<Boolean>(false)
onMounted(() => {
  fileDiff(props.repo, props.file.prevObjectId, props.file.objectId).then(async res => {
    diffDetailLines = findDiffInfoLine(res.display)
    const lines = splitModifLines(res.display)
    addedLines = lines.added
    deletedLines = lines.deleted
    res.display = lines.content
    diffContent.value = res
    success.value = true
    await nextTick()
    initEditor()
    decorations = applyEditorStyle()
  }).catch(err => {
    console.error(err)
    success.value = false
  })
})

const added = computed(() => {
 return diffContent.value?.ops.reduce((acc, opt) => {
  if (opt.op == 'insert' || opt.op == 'replace') return acc + opt.new_len
  return acc
 }, 0) || 0
})
const deleted = computed(() => {
  return diffContent.value?.ops.reduce((acc, opt) => {
    if (opt.op == 'delete' || opt.op =='replace') return acc + opt.old_len
    return acc
   }, 0) || 0
})

const modifRatiStyle = computed(() => {
  const a = added.value / (added.value + deleted.value)
  return {
    background: `linear-gradient(to right, #4ade80 ${a * 100}%, #f87171 ${a * 100}%)`,
  }
})

const editorContainer = ref<HTMLElement>();
let editor: monaco.editor.IStandaloneCodeEditor;

function calculateEditorHeight() {
  const lineHeight = editor.getOption(monaco.editor.EditorOption.lineHeight);
  const lineCount = editor.getModel()!.getLineCount();
  const scrollBeyondLastLine = editor.getOption(monaco.editor.EditorOption.scrollBeyondLastLine);
  const scrollHeight = editor.getDomNode()!.scrollHeight;

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

const initEditor = () => {
  if (!editorContainer.value) return;
  editor = monaco.editor.create(editorContainer.value, {
    value: diffContent.value?.display || '',
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
  });
  updateEditorHeight()
};

// 找到diff信息行
const findDiffInfoLine = (content: String) => {
  const reg = /^@@ -(\d*),(\d*) \+(\d*),(\d*) @@$/
  const n = []
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
  const addedDecorations: monaco.editor.IModelDeltaDecoration[] = addedLines.map((lineNumber: number) => {
    return {
      range: new monaco.Range(lineNumber + 1, 1, lineNumber + 1, 1),
      options: {
        isWholeLine: true,
        className: 'added-line',
      }
    }
  });
  const deletedDecorations: monaco.editor.IModelDeltaDecoration[] = deletedLines.map((lineNumber: number) => {
    return {
      range: new monaco.Range(lineNumber + 1, 1, lineNumber + 1, 1),
      options: {
        isWholeLine: true,
        className: 'deleted-line', 
      } 
    }
  })
  const diffDetailDecorations: monaco.editor.IModelDeltaDecoration[] = diffDetailLines.map((lineNumber: number) => {
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
  <NCard>
    <template #header>
      <div>
        {{ file.path }}
      </div>
    </template>
    <template #header-extra>
      <div class="flex gap-1 items-center">
        <div class="text-green-400">
          {{ + added  }}
        </div>
        <div class="text-red-400">
          {{ + deleted  }}
        </div>
        <div class="h-[10px] w-[30px]" :style="modifRatiStyle"></div>
      </div>
    </template>
    <div v-if="success" ref="editorContainer"></div>
    <div v-else>
      加载失败
    </div>
  </NCard>
</template>


<style>
.added-line {
  background: #abfac8;
}
.deleted-line {
  background: #faa3a3;
}
.diff-detail-line {
  background: #93d1f5;
}
</style>