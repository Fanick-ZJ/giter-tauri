<script setup lang="ts">
import dayjs from 'dayjs';
import { FileHistoryItem } from '@/types';
import { Icon } from '@iconify/vue'
import { NFlex, NGrid, NSpace, NLayout, NScrollbar, NEllipsis, NButton, NCol, NGi } from 'naive-ui';
import DiffEditor from '@/components/common/editor/diffEditor.vue';
import { computed, Ref, ref, watch } from 'vue';
import { getBlobContent } from '@/utils/command';
import { bytesToString } from '@/utils/tool';
import { FileOption } from '@/components/common/editor/types';

const props = defineProps<{
    repo: string,
    historyList: FileHistoryItem[],
    tagretId: string,
    currentId: string
}>()

const useHistoryItem = () => {
    const targetItem = computed(() => {
        return props.historyList.find(item => item.commit.commitId == props.tagretId)
    })
    const messageShow = ref(false)
    const currentItem = computed(() => {
        return props.historyList.find(item => item.commit.commitId == props.currentId)
    })

    const original: Ref<FileOption> = ref({
      language: undefined,
      content: '',
      filename: undefined
    })
    const modified: Ref<FileOption> = ref({
      language: undefined,
      content: '',
      filename: undefined
    })
    watch(() => props.tagretId, async() => {
      original.value.filename = targetItem.value?.file.path
      return getBlobContent(props.repo, targetItem.value!.file.objectId).then((res) => {
        original.value.content = bytesToString(res)
      }).catch((err) => {
        window.$message.error('获取历史比较对象文件失败')
        return 'THIS FILE IS NOT FOUND'
      })
    }, {immediate: true})
    
    watch(() => props.currentId, async() => {
      modified.value.filename = currentItem.value?.file.path
      return getBlobContent(props.repo, currentItem.value!.file.objectId).then((res) => {
        modified.value.content = bytesToString(res)
      }).catch((err) => {
        window.$message.error('获取历史比较对象文件失败')
        return 'THIS FILE IS NOT FOUND'
      })
    }, {immediate: true})


    const prevHistoryHandler = () => {
      const index = props.historyList.indexOf(targetItem.value!)
      if (index == 0) {
        window.$message.info('已经是最近的记录啦')
      } else {
        emit('updateTargetId', props.historyList[index - 1].commit.commitId)
      }
    }

    const nextHistoryHandler = () => {
      const index = props.historyList.indexOf(targetItem.value!)
      if (index == props.historyList.length - 1) {
        window.$message.info('已经是最迟的记录啦')
      } else {
        emit('updateTargetId', props.historyList[index + 1].commit.commitId)
      }
    }

    return {
        targetItem,
        messageShow,
        currentItem,
        original,
        modified,
        prevHistoryHandler,
        nextHistoryHandler
    }
}
const {
  targetItem, 
  messageShow, 
  currentItem,
  original,
  modified,
  prevHistoryHandler,
  nextHistoryHandler
} = useHistoryItem()

const emit = defineEmits<{
    (e:'updateTargetId', val: string): void
    (e:'exit'): void
}>()

</script>

<style scoped>

</style>

<template>
  <DiffEditor 
    :original="original"
    :modified="modified"
    :readonly="true"
  >
    <template #header>
      <NGrid :x-gap="12" :cols="2">
        <NGi v-if="targetItem != undefined">
          <!-- 头部标题和按钮 -->
          <NFlex :justify="'space-between'" :align="'center'" class="px-1 mb-2" ref="header">
            <div>
              <NEllipsis style="max-width:300px">
              {{ targetItem.commit.title }}
              </NEllipsis>
            </div>
            <NFlex>
              <NButton class="h-[25px]" @click="prevHistoryHandler" circle>
                <template #icon>
                    <Icon icon="typcn:chevron-left" width="24" height="24" />
                </template>
              </NButton>
              <NButton class="h-[25px]" @click="nextHistoryHandler" circle>
                <template #icon>
                    <Icon icon="typcn:chevron-right" width="24" height="24" />
                </template>
              </NButton>
              <NButton class="h-[25px]" @click="messageShow = !messageShow" circle>
                <template #icon>
                    <Icon v-if="messageShow" icon="mage:message-dots" width="24" height="24" />
                    <Icon v-else icon="eva:arrow-down-outline" width="24" height="24" />
                </template>
              </NButton>
            </NFlex>
          </NFlex>
          <NLayout content-class="px-1">
            <NScrollbar 
                :style="{maxHeight: messageShow ? '100px' : '0px'}" 
                class="transition-all duration-300"
            >
                {{ targetItem.commit.message }}
            </NScrollbar>
            <NFlex :justify="'space-between'">
                <div class="text-lg dark:text-gray-400">
                {{ targetItem.commit.authorName }}
                </div>
                <div class="text-sm dark:text-gray-400">
                {{ dayjs(targetItem.commit.datetime).format('YYYY-MM-DD HH:mm:ss') }}
                </div>
            </NFlex>
          </NLayout>
        </NGi>
        <NGi v-else :justify="'center'" :align="'center'">
          没找到此目标文件
        </NGi>
        <NGi v-if="currentItem">
          <!-- 头部标题和按钮 -->
          <NFlex :justify="'space-between'" :align="'center'" class="px-1 mb-2" ref="header">
            <div>
              <NEllipsis style="max-width:300px">
              {{ currentItem.commit.title }}
              </NEllipsis>
            </div>
            <NFlex>
              <NButton class="h-[25px]" @click="messageShow = !messageShow" circle>
                <template #icon>
                    <Icon v-if="messageShow" icon="mage:message-dots" width="24" height="24" />
                    <Icon v-else icon="eva:arrow-down-outline" width="24" height="24" />
                </template>
              </NButton>
              <NButton class="h-[25px]" @click="() => emit('exit')" circle>
                <template #icon>
                    <Icon icon="mingcute:exit-line" width="24" height="24" />
                </template>
              </NButton>
            </NFlex>
          </NFlex>
          <NLayout content-class="px-1">
            <NScrollbar 
                :style="{maxHeight: messageShow ? '100px' : '0px'}" 
                class="transition-all duration-300"
            >
                {{ currentItem.commit.message }}
            </NScrollbar>
            <NFlex :justify="'space-between'">
                <div class="text-lg dark:text-gray-400">
                {{ currentItem.commit.authorName }}
                </div>
                <div class="text-sm dark:text-gray-400">
                {{ dayjs(currentItem.commit.datetime).format('YYYY-MM-DD HH:mm:ss') }}
                </div>
            </NFlex>
          </NLayout>
        </NGi>
        <NGi v-else :justify="'center'" :align="'center'">
          没找到此目标文件
        </NGi>
      </NGrid>
    </template>
  </DiffEditor>
</template>