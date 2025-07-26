<script setup lang="ts">
import Editor from '@/components/common/editor/editor.vue';
import HistoryComparePanel from './history-compare-panel.vue'
import { Icon } from '@iconify/vue'
import { NTabs, NTabPane, NButton } from 'naive-ui'
import { ref, watch, nextTick } from 'vue';
import { FileHistoryEventData } from '@/windows/file-history';
import { getBlobContent } from '@/utils/command';
import { FileHistoryItem } from '@/types';
import { bytesToString } from '@/utils/tool';
import { createCompareSelectDialog } from './compare-select-dialog';
import HistoryHeader from './history-header.vue';

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
  const showMessageHandler = () => {
    showMessage.value = !showMessage.value
  }
  return {
    showMessage,
    showMessageHandler
  }
}
const { showMessage, showMessageHandler } = useStyle()

const useCompareHistory = () => {
  const comparedHistory = ref<FileHistoryItem>()
  const isComparing = ref(false)
  const compareHistory = async (history: FileHistoryItem) => {
    await nextTick()
    createCompareSelectDialog({
      historyList: props.history.history,
    }).then( res => {
      comparedHistory.value = res
      isComparing.value = true
    })
  }
  
  const compareEnd = () => {
    isComparing.value = false
    comparedHistory.value = undefined
  }
  
  const updateTargetHandle = (targetId: string) => {
    props.history.history.forEach(item => {
      if (item.commit.commitId == targetId) {
        comparedHistory.value = item
      }
    })
  }

  return {
    compareHistory,
    compareEnd,
    isComparing,
    comparedHistory,
    updateTargetHandle
  }
}
const {
  compareHistory,
  compareEnd,
  isComparing,
  comparedHistory,
  updateTargetHandle,
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
  <div class="h-full">
    <NTabs
      v-if="!isComparing"
      :key="history.path"
      v-model:value="curCommit"
      :style="{height: `${props.height}px`}"
      :tab-style="{width: '200px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap'}"
      placement="left"
      ref="tabsRef"
      @before-leave="handlePaneChange"
      :type="'line'"
    >
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
            <HistoryHeader :history="item" :show-message="showMessage" @show-message-click="showMessageHandler">
              <template #right-action>
                <NButton class="h-[25px]" circle @click="compareHistory(item)">
                  <template #icon>
                    <Icon icon="iconamoon:compare-bold" width="24" height="24" />
                  </template>
                </NButton>
              </template>
            </HistoryHeader>
          </template>
        </Editor>
      </NTabPane>
    </NTabs>
    <HistoryComparePanel
      v-else 
      :repo="history.repo"
      :current-id="curCommit" 
      :tagret-id="comparedHistory!.commit.commitId" 
      :history-list="history.history"
      @update-target-id="updateTargetHandle"
      @exit="compareEnd"
    >
    </HistoryComparePanel>
  </div>
</template>