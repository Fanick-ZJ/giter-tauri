<script setup lang="ts">
import { computed, CSSProperties, inject, nextTick, onBeforeUnmount, onMounted, PropType, Ref, ref, shallowRef, watch } from 'vue';
import { DiffContent, CommitEntry } from '@/types';
import { Icon } from '@iconify/vue';
import { NCard, NWatermark, NFlex, NButton, useDialog } from 'naive-ui';
import * as monaco from 'monaco-editor';
import { bytesToString, getMonacoLanguage, withMinDelay } from '@/utils/tool';
import { showFileHistory } from '@/utils/dialog';
import LoadingView from '@/components/common/loading-view.vue';
import { fileDiff, getBlobContent, objectIsBinary } from '@/utils/command';
import { BinaryResult, processBinaryData } from './utils';
import { commitIdKey } from './keys';
import { useThemeStore } from '@/store/modules/theme'
import { EntryMode } from '@/enum';

defineOptions({
  name: 'DiffDetailComponent' 
})
const dialog = useDialog()
const props = defineProps({
  repo: {
    type: String,
    required: true 
  },
  file: {
    type: Object as PropType<CommitEntry>,
    required: true
  }
})

const commitId = inject(commitIdKey)
const diffContent = ref<DiffContent>()
let addedLines: Ref<number[]> = ref([])
let deletedLines: Ref<number[]> = ref([])
let diffDetailLines: Ref<number[]> = ref([])
const success = ref<boolean>(false)
const loading = ref<boolean>(true)
const binaryComps:Ref<BinaryResult> = shallowRef([undefined, undefined])

// 暴露给外部调用，动态加载，避免拥堵
const load = async () => {
  if (await objectIsBinary(props.repo, props.file.objectId)) {
    processBinaryData(props.repo, props.file)!.then(res => {
      binaryComps.value = res
      success.value = true	
    }).finally(() => {
      loading.value = false	
    })
    success.value = false
    return
  }

  loading.value = false
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
    })
  }
  else if (props.file.status === 'Deleted') {
    getBlobContent(props.repo, props.file.prevObjectId).then(async res => {
      // 将u8数组转换为字符串
      const content = bytesToString(res)
      success.value = true
      deletedLines.value = content.split('\n').map((_, i) => i)

      await nextTick()
      initEditor(content)
      applyEditorStyle()
    }).catch(err => {
      console.error(err)
      success.value = false
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
    })
  }
}


const modifRatiStyle = computed(() => {
  const a = addedLines.value.length / (addedLines.value.length + deletedLines.value.length)
  return {
    background: `linear-gradient(to right, #4ade80 ${a * 100}%, #f87171 ${a * 100}%)`,
  }
})

// 将 isBinary 改为响应式引用
const isBinary = ref<boolean>(false)
const isBinaryLoading = ref<boolean>(true)

// 在组件初始化时检查是否为二进制文件
const checkIsBinary = async () => {
  try {
    isBinaryLoading.value = true
    const result = await objectIsBinary(props.repo, props.file.objectId)
    isBinary.value = result
  } catch (error) {
    console.error('检查二进制文件失败:', error)
    isBinary.value = false
  } finally {
    isBinaryLoading.value = false
  }
}

// 在组件挂载时检查
onMounted(() => {
  console.log(props.file.path, props.file.entryMode)
  checkIsBinary()
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

const themeStore = useThemeStore()
// 监听主题变化
watch(() => themeStore.isDark, (isDark) => {
  if (editor) {
    monaco.editor.setTheme(isDark ? 'vs-dark' : 'vs')
  }
})

const initEditor = (content: string) => {
  if (!editorContainer.value) return;
  editor = monaco.editor.create(editorContainer.value, {
    value: content,
    language: getMonacoLanguage(props.file.path),
    theme: themeStore.isDark ? 'vs-dark' : 'vs', // 根据主题设置
    minimap: {
      enabled: false,
    },
    scrollBeyondLastLine: false,
    automaticLayout: true,
    scrollbar: {
      vertical: 'hidden',
      horizontal: 'auto',
      handleMouseWheel: true,
      alwaysConsumeMouseWheel: false,
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

const headerStyle = computed<CSSProperties>(() => {
  return {
    position: 'sticky', 
    top: '-1px', 
    zIndex: 5, 
    backgroundColor: themeStore.isDark 
      ? 'rgba(24, 24, 28, 0.9)' 
      : 'rgba(255, 255, 255, 0.9)' 
  }
})

</script>

<template>
  <NCard :header-style="headerStyle">
    <template #header>
      <div>
        {{ file.path }}
      </div>
    </template>
    <template #header-extra>
      <!-- 直接使用响应式变量，无需 await -->
      <div v-if="!isBinary" class="flex gap-1 items-center">
        <div class="text-green-400">
          {{ + addedLines.length  }}
        </div>
        <div class="text-red-400">
          {{ + deletedLines.length  }}
        </div>
        <div class="h-[10px] w-[30px]" :style="modifRatiStyle"></div>
        <NButton quaternary @click="() => showFileHistory(dialog, repo, file.path, commitId)">
          <template #icon>
            <Icon icon="material-symbols:history-rounded" width="24" height="24" />
          </template>
          文件历史
        </NButton>
      </div>
    </template>
    <LoadingView :loading="loading">
      <div>
        <!-- 直接使用响应式变量 -->
        <div v-if="success && !isBinary" ref="editorContainer"></div>
        <div v-else-if="isBinary">
          <NFlex>
            <div class="flex-1 gap-2">
              <NWatermark 
                content="NEW" 
                selectable 
                cross 
                class="z-[2] w-full h-full flex justify-center items-center"
                :x-offset="12"
                :y-offset="12"
                :width="100"
                :height="50"
                :rotate="-15">
                <component v-if="binaryComps[0]" :is="binaryComps[0].name" v-bind="binaryComps[0].param"/>
                <div v-else>
                  新的数据没有了喔😊
                </div>
              </NWatermark>
            </div>
            <div class="flex-1">
              <NWatermark 
                content="OLD" 
                selectable 
                cross 
                class="z-[2] w-full h-full flex justify-center items-center"
                :x-offset="12"
                :y-offset="12"
                :width="100"
                :height="50"
                :rotate="-15">
                <component v-if="binaryComps[1]" :is="binaryComps[1].name" v-bind="binaryComps[1].param"/>
                <div v-else class="text-center text-lg font-medium text-gray-900 italic dark:text-gray-200 ">
                  没有旧的数据喔😊
                </div>
              </NWatermark>
            </div>
          </NFlex>
        </div>
        <div v-else-if="!loading && !success">
          加载失败
        </div>
      </div>
    </LoadingView>
  </NCard>
</template>


<style scoped>
</style>