<script setup lang="ts">
import dayjs from 'dayjs';
import Editor from '@/components/common/editor/editor.vue';
import { FileHistoryItem } from '@/types';
import { getBlobContent } from '@/utils/command';
import { bytesToString, getMonacoLanguage } from '@/utils/tool';
import { FileHistory } from '@/windows/file-history';
import { NTabs, NTabPane, NSpace } from 'naive-ui'
import { ref } from 'vue';

const props = defineProps<{
  history: FileHistory,
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

const handlePaneChange = async (name: string) => {
  const history = props.history.history.find(item => item.commit.commitId === name)
  if (!history) {
    window.$message.error('文件历史记录已被未找到')
  }
  currentHistoryFileContent.value = await getHistoryContent(history!)
  return true
}
</script>

<template>
  <NSpace vertical class="h-full">
    <NTabs
      :key="history.path"
      v-model:value="curCommit"
      :style="{height: `${props.height}px`}"
      :tab-style="{width: '200px'}"
      placement="left"
      @before-leave="handlePaneChange"
      :type="'line'">
      <NTabPane
        v-for="(item, index) in history.history"
        :key="item.commit.commitId"
        :closable="true"
        :name="item.commit.commitId"
        :tab="item.commit.message">
        <Editor 
          :language="getMonacoLanguage(props.history.path)" 
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