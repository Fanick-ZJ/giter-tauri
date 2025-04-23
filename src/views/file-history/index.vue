<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useMessage, NLayout, NTabs, NTabPane, NLayoutSider } from 'naive-ui'
import { computed, onMounted, provide, ref, watch } from 'vue';

import FileHistoryWindow, { FileHistory, LOCAL_STORAGE_FIRST_FILE_HISTORY } from '@/windows/file-history';
import { HistoryTabInjectKey } from './types';
import HistoryPanel from './history-panel.vue';
import { basename } from '@/utils/tool';
import { useElementSize } from '@vueuse/core';

const historyTab = ref<FileHistory[]>([])
window.$message = useMessage()

const KEY_INTERVAL = "<KEY_INTERVAL>"
provide(HistoryTabInjectKey, historyTab)

onMounted(() => {
  const history = localStorage.getItem(LOCAL_STORAGE_FIRST_FILE_HISTORY);
  if (history) {
    const historys = JSON.parse(history) as FileHistory
    historyTab.value.push(historys)
    curPanel.value = historyKey(historys)
  }
})

const curWindow = getCurrentWindow();
curWindow.listen<FileHistory>(FileHistoryWindow.FILE_ADD, (evt) => {
  const { payload } = evt;
  const index = historyTab.value.findIndex(item => item.path === payload.path && item.repo === payload.repo)
  if (index === -1) {
    historyTab.value.push(payload)
  } else {
    curPanel.value = historyKey(payload)
  }
  curPanel.value = historyKey(payload)

})

const curPanel = ref<string>('')

const closeHandler = (key: string) => {
  const index = historyTab.value.findIndex(item => historyKey(item) === key)
  if (index !== -1) {
    historyTab.value.splice(index, 1)
  }
  if (historyTab.value.length === 0) {
    curPanel.value = ''
  } else {
    if (index + 1 < historyTab.value.length) {
      curPanel.value = historyKey(historyTab.value[index + 1])
    } else {
      curPanel.value = historyKey(historyTab.value[index - 1])
    }
  }
}

const historyKey = (history: FileHistory) => {
  return history.repo + KEY_INTERVAL + history.path 
}

const getHistoryByKey = (key: string) => {
  const [repo, path] = key.split(KEY_INTERVAL)
  return historyTab.value.find(item => item.path === path && item.repo === repo) 
}

const containerRef = ref<HTMLElement | null>(null)
const { width, height } = useElementSize(containerRef)
const tabbarHeight = ref(45)

const panelHeight = computed(() => {
  return height.value - tabbarHeight.value - 15
})

</script>
<template>
  <NLayout class="h-screen" :style="{height: '100vh'}" ref="containerRef">
    <NTabs
      v-model:value="curPanel"
      type="card"
      closable
      :tab-style="{ height: `${tabbarHeight}px`, overflow: 'auto' }"
      :tab-padding="0"
      :tab-size="0"
      placement="top"
      class="flex h-screen pt-1"
      animated
      :pane-wrapper-style="{flex: '1 auto'}"
      @close="closeHandler">
      <NTabPane
        v-for="(item, index) in historyTab"
        :key="index"
        :closable="true"
        :name="historyKey(item)"
        :tab="basename(item.path)"
        :show-close="true">
        <HistoryPanel :history="item" :height="panelHeight"/>
      </NTabPane>
    </NTabs>
  </NLayout>
</template>