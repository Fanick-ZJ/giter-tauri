<script setup lang="ts">
import dayjs from 'dayjs';
import Editor from '@/components/common/editor/editor.vue';
import { FileHistoryItem } from '@/types';
import { getBlobContent } from '@/utils/command';
import { bytesToString, getMonacoLanguage } from '@/utils/tool';
import { NTabs, NTabPane, NSpace, NEllipsis } from 'naive-ui'
import { nextTick, ref, watch } from 'vue';
import { FileHistoryEventData } from '@/windows/file-history';

const props = defineProps<{
  history: FileHistoryEventData,
  height: number,
}>()

const curCommit = ref('')
const currentHistoryFileContent = ref('')
const getHistoryContent = (history: FileHistoryItem) => {
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
watch(() => props.history.focusCommit, async (val) => {
  curCommit.value = val || props.history.history[0].commit.commitId
  handlePaneChange(curCommit.value)
}, {
  immediate: true,
})
</script>

<template>
  <NSpace vertical class="h-full">
    <NTabs
      :key="history.path"
      v-model:value="curCommit"
      :style="{height: `${props.height}px`}"
      :tab-style="{width: '200px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap'}"
      placement="left"
      @before-leave="handlePaneChange"
      :type="'line'">
      <NTabPane
        v-for="(item, index) in history.history"
        :key="item.commit.commitId"
        :closable="true"
        :name="item.commit.commitId"
        :tab="item.commit.message">
        <!-- @vue-ignore -->
        <template #tab>
          <NEllipsis style="max-width:180px">
            {{ item.commit.message }}
          </NEllipsis>
        </template>
        <Editor 
          :filename="item.file.path"
          :content="currentHistoryFileContent"
          :readonly="true">
          <template #header>
            <div class="flex justify-between px-6">
              <div>
                <span class="text-lg">
                {{ item.commit.authorName }}
              </span>
              <span class="text-sm text-gray-500 ml-2">
                {{ item.commit.title }}
              </span>
              </div>
              <span class="text-sm text-gray-500 ml-2">
                {{ dayjs(item.commit.datetime).format('YYYY-MM-DD HH:mm:ss') }}
              </span>
            </div>
          </template>
        </Editor>
      </NTabPane>
    </NTabs>
  </NSpace>
</template>