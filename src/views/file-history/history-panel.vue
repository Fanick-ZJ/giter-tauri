<script setup lang="ts">
import dayjs from 'dayjs';
import Editor from '@/components/common/editor/editor.vue';
import { Icon } from '@iconify/vue'
import { NTabs, NTabPane, NSpace, NSpin, NFlex, NButton, NText, NEllipsis, NLayout, NScrollbar } from 'naive-ui'
import { ref, watch, computed, nextTick, onMounted } from 'vue';
import { FileHistoryEventData } from '@/windows/file-history';
import { getBlobContent } from '@/utils/command';
import { FileHistoryItem } from '@/types';
import { bytesToString } from '@/utils/tool';
import { createCompareSelectDialog } from './compare-select-dialog';

const props = defineProps<{
  history: FileHistoryEventData,
  height: number,
}>()

const useTabHandler = () => {
  const curCommit = ref('')
  const tabsRef = ref<InstanceType<typeof NTabs>>()
  const currentHistoryFileContent = ref('')
  const getHistoryContent = async (history: FileHistoryItem) => {
    return getBlobContent(props.history.repo, history.file.objectId).then((res) => {
      const content = bytesToString(res)
      return content
    }).catch((err) => {
      console.error(err)
      return 'THIS FILE IS NOT FOUND'
    })
  }

  const handlePaneChange = async (commitId: string) => {
    const history = props.history.history.find(item => item.commit.commitId === commitId)
    if (!history) {
      window.$message.error('文件历史记录已被未找到')
    }
    currentHistoryFileContent.value = await getHistoryContent(history!)
    return true
  }
  return {
    tabsRef,
    curCommit,
    currentHistoryFileContent,
    handlePaneChange,
  }
}
const {
  tabsRef,
  curCommit,
  currentHistoryFileContent,
  handlePaneChange,
} = useTabHandler()

const useStyle = () => {
  const showMessage = ref(false)

  return {
    showMessage,
  }
}
const { showMessage } = useStyle()

const useCompareHistory = () => {
  const comparedHistory = ref<FileHistoryItem>()
  const isComparing = ref(false)
  const compareHistory = async (history: FileHistoryItem) => {
    isComparing.value = true
    comparedHistory.value = history
    await nextTick()
    createCompareSelectDialog({
      historyList: props.history.history,
    }).then(res => {
      console.log(res)
    })
  }
  const compareEnd = () => {
    isComparing.value = false
    comparedHistory.value = undefined
  }
  return {
    compareHistory,
    compareEnd,
    isComparing,
    comparedHistory,
  }
}
const {
  compareHistory,
  compareEnd,
  isComparing,
  comparedHistory,
} = useCompareHistory()


watch(() => props.history.focusCommit, async (val) => {
  if (!val) {
    return
  }
  await initializeCommit(val)
  curCommit.value = val
  nextTick(() => {
    tabsRef.value?.syncBarPosition()
    // 自动滚动到对应的 tab 位置
    scrollToTab(val)
  })
}, {
  immediate: true,
})

async function initializeCommit(val: string | undefined) {
  if (!val) {
    return
  }
  await handlePaneChange(val)
}

function scrollToTab(commitId: string) {
  nextTick(() => {
    // 等待 DOM 更新后再查找元素
    setTimeout(() => {
      // 尝试多种选择器来找到对应的 tab 元素
      let tabElement = document.querySelector(`[data-name="${commitId}"]`) as HTMLElement
      
      // 如果第一种方式找不到，尝试其他方式
      if (!tabElement && tabsRef.value) {
        const tabsContainer = tabsRef.value.$el
        // 查找包含 commitId 的 tab 元素
        const allTabs = tabsContainer.querySelectorAll('.n-tab')
        for (const tab of allTabs) {
          if (tab.getAttribute('data-name') === commitId || 
              tab.textContent?.includes(commitId.substring(0, 8))) {
            tabElement = tab as HTMLElement
            break
          }
        }
      }
      
      if (tabElement) {
        // 滚动到该 tab 位置
        tabElement.scrollIntoView({
          behavior: 'smooth',
          block: 'nearest',
          inline: 'center'
        })
      }
    }, 100)
  })
}
</script>

<template>
  <NSpace vertical class="h-full">
    <NTabs
      :key="history.path"
      v-model:value="curCommit"
      :style="{height: `${props.height}px`}"
      :tab-style="{width: '200px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap'}"
      placement="left"
      ref="tabsRef"
      @before-leave="handlePaneChange"
      :type="'line'">
      <NTabPane
        v-for="(item, index) in history.history"
        :key="item.commit.commitId"
        :closable="true"
        :name="item.commit.commitId"
        :tab="item.commit.message"
      >
        <Editor 
          :filename="item.file.path"
          :content="currentHistoryFileContent"
          :readonly="true">
          <template #header>
            <NFlex :justify="'space-between'" :align="'center'" class="px-1 mb-2" ref="header">
              <div>
                <NEllipsis style="max-width:300px">
                  {{ item.commit.title }}
                </NEllipsis>
              </div>
              <NFlex>
                <NButton class="h-[25px]" circle @click="compareHistory(item)">
                  <template #icon>
                    <Icon icon="iconamoon:compare-bold" width="24" height="24" />
                  </template>
                </NButton>
                <NButton class="h-[25px]" @click="showMessage = !showMessage" circle>
                  <template #icon>
                    <Icon v-if="showMessage" icon="mage:message-dots" width="24" height="24" />
                    <Icon v-else icon="eva:arrow-down-outline" width="24" height="24" />
                  </template>
                </NButton>
              </NFlex>
            </NFlex>
            <NLayout content-class="px-1">
              <NScrollbar 
                :style="{maxHeight: showMessage ? '100px' : '0px'}" 
                class="transition-all duration-300"
              >
                {{ item.commit.message }}
              </NScrollbar>
              <NFlex :justify="'space-between'">
                <div class="text-lg dark:text-gray-400">
                  {{ item.commit.authorName }}
                </div>
                <div class="text-sm dark:text-gray-400">
                  {{ dayjs(item.commit.datetime).format('YYYY-MM-DD HH:mm:ss') }}
                </div>
              </NFlex>
            </NLayout>
          </template>
        </Editor>
      </NTabPane>
    </NTabs>
  </NSpace>
</template>