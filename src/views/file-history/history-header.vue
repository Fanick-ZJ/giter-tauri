<script setup lang="ts">
import dayjs from 'dayjs';
import { Icon } from '@iconify/vue'
import { FileHistoryItem } from '@/types';
import { NFlex, NLayout, NScrollbar, NEllipsis, NButton, NCard, useMessage } from 'naive-ui';
import { getCurrentInstance, PropType, ref } from 'vue';
import { template } from 'lodash';

const props = defineProps({
  history: {
    type: Object as PropType<FileHistoryItem>,
    required: true
  },
  showMessage: {
    type: Boolean,
    required: true
  }
})
const message = useMessage()
const instance = getCurrentInstance()
const commitClickHandler = () => {
  const text = navigator.clipboard
  text.writeText(props.history.commit.commitId)
  message.success('已复制commit id到剪贴板')
}
const emit = defineEmits<{
  (e: 'showMessageClick'): void
}>()
</script>

<style scoped>

</style>

<template>
<div class="px-2 flex flex-col relative h-full">
  <NCard size="small">
    <template #header>
      <NFlex :justify="'space-between'" :align="'center'" ref="header">
        <div>
          <NEllipsis style="max-width:300px">
            {{ history.commit.title }}
          </NEllipsis>
        </div>
        <NFlex>
          <slot name="left-action"></slot>
          <NButton class="h-[25px]" @click="() => emit('showMessageClick')" circle>
            <template #icon>
              <Icon v-if="showMessage" icon="mage:message-dots" width="24" height="24" />
              <Icon v-else icon="eva:arrow-down-outline" width="24" height="24" />
            </template>
          </NButton>
          <slot name="right-action"></slot>
        </NFlex>
      </NFlex>
    </template>  
    <NScrollbar 
      :style="{maxHeight: showMessage ? '100px' : '0px'}" 
      class="transition-all duration-300"
    >
      {{ history.commit.message }}
    </NScrollbar>
    <template #footer>
      <div class="flex justify-between items-end">
        <div class="text-lg dark:text-gray-400">
          {{ history.commit.authorName }}
        </div>
        <div class='flex justify-between items-center gap-2'>    
            <NButton type="info" dashed size='small' @click=commitClickHandler>
                {{history.commit.commitId.slice(0, 8)}}
            </NButton>
            <div class="text-sm dark:text-gray-400">
            {{ dayjs(history.commit.datetime).format('YYYY-MM-DD HH:mm:ss') }}
            </div>
        </div>
      </div>
    </template>
  </NCard>
</div>
</template>