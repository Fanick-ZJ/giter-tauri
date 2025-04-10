<script setup lang="tsx">
import { computed, nextTick, onBeforeUnmount, onMounted, StyleValue, useTemplateRef, watch } from 'vue';
import { Commit, CommitFile } from '@/types';

import { ref } from 'vue';
import { NCard, NFlex, NLayout, NTag, NPagination } from 'naive-ui';
import { commitContent, fileDiff, getCommit } from '@/utils/command';
import { useElementSize } from '@vueuse/core';
import DiffDetailComponent from './diff-detail-item.vue';

defineOptions({
  name: 'CommitDetailComponent'
})

const unmount = ref<Function>()
const props = defineProps({
  commitId: {
    type: String,
    required: true
  },
  repo: {
    type: String,
    required: true
  }
})

const commitFiles = ref<CommitFile[]>()
const commit = ref<Commit>()

// 懒加载, 滚动到可视区域再加载, 避免卡顿
const diffDetailRefs = ref<InstanceType<typeof DiffDetailComponent>[]>([])
let observer: IntersectionObserver;

const obserAll = () => {
  if (observer) {
    observer.disconnect() 
  }
  observer = new IntersectionObserver(entries => {
  entries.forEach(entry => {
    if (entry.isIntersecting) {
      const index = diffDetailRefs.value.findIndex(item => item.$el === entry.target)
      diffDetailRefs.value[index].$.exposed!.load()
      observer.unobserve(entry.target)
    }
  })
})
  diffDetailRefs.value.forEach(item => {
    observer.observe(item.$el)
  }) 
}

onMounted(async () => {
  const res = await Promise.allSettled([getCommit(props.repo, props.commitId), commitContent(props.repo, props.commitId)])
  if (res[0].status === 'fulfilled') {
    commit.value = res[0].value 
  } else {
    window.$message.error('获取提交信息失败')
  }
  if (res[1].status === 'fulfilled') {
    commitFiles.value = res[1].value 
  } else {
    window.$message.error('获取提交内容失败') 
  }
  // 设置滚动条的z-index,在layout上设置了style无效
  containerRef.value!.querySelector('.n-scrollbar-rail')!.setAttribute('style', 'z-index: 4')
  await nextTick()
  obserAll()
})

onBeforeUnmount(() => {
  diffDetailRefs.value.forEach(item => {
    observer.observe(item.$el)
  }) 
  commitFiles.value = []
  observer.disconnect()
})

const __show = ref<Boolean>(false)
let resolve: any
let reject: any
let show = () => {
  __show.value = true
  return new Promise((res, rej) => {
    resolve = res
    reject = rej
  })
}

const close = () => {
  __show.value = false
  unmount.value && unmount.value()
  resolve()
}

defineExpose({
  setUnmount: (fn: Function) => {
    unmount.value = fn
  },
  show,
  close
})

// 为了滚动框高度自适应
const containerRef = ref<HTMLElement>()
const size = useElementSize(containerRef)
const containerStyle = ref<StyleValue>()
watch(size.height, async () => {
  await nextTick()
  containerStyle.value = {
    height: size.height.value + 'px'
  } 
})

const contentRef = useTemplateRef('contentRef')
const page = ref(1)
const pageSize = ref(10)
const handlePageChange = (page: number) => {
  contentRef.value?.scrollTo(0, 0)
}

const pageItems = computed(() => {
  return commitFiles.value?.slice((page.value - 1) * pageSize.value, page.value * pageSize.value) || [] 
})

watch(() => page.value, async () => {
  await nextTick()
  obserAll() 
})

</script>

<template>
  <div v-if="__show" 
    class="w-screen h-screen 
    bg-slate-400/50 flex 
    items-center justify-center 
    fixed top-0 
    left-0 z-[3]">
    <NCard title="提交详情" class="w-[80%] h-[80%]" closable @close="close">
      <template #header-extra>
        <div class="flex gap-3">
          <div class="flex items-center">
            <span class="font-medium text-gray-600">
              父节点
            </span>
            <div class="flex gap-1">
              <template v-for="item in commit?.parents">
                <NTag :type="'info'" class="ml-2">
                  {{item.slice(0, 7)}}
                </NTag>
              </template>
            </div>
          </div>
          <div class="flex items-center">
            <span class="font-medium text-gray-600">
              当前节点
            </span>
            <NTag :type="'success'" class="ml-2">
              {{commit?.commitId.slice(0, 7)}}
            </NTag>
          </div>
        </div>
      </template>
      <div class="h-full relative" ref="containerRef">
        <NLayout 
          ref="contentRef"
          class="absolute w-full"
          :style="containerStyle" 
          :native-scrollbar="false"
          >
          <NFlex justify="center">
            <template v-for="item in pageItems" :key="item.objectId">
              <DiffDetailComponent ref="diffDetailRefs" :repo="repo" :file="item" />
            </template>
            <NPagination
              v-if="(commitFiles?.length || 0) > pageSize"
              :item-count="commitFiles?.length"
              v-model:page="page"
              v-model:page-size="pageSize"
              :page-sizes="[10, 20, 30]"
              @update-page="handlePageChange" 
              show-size-picker/>
          </NFlex>
        </NLayout>
      </div>
    </NCard>
  </div>
</template>


<style scoped>
</style>