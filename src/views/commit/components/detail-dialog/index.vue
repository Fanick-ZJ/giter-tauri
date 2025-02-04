<script setup lang="ts">
import { computed, nextTick, onBeforeMount, onMounted, StyleValue, useTemplateRef, watch } from 'vue';
import { File } from '@/types';

import { ref } from 'vue';
import { NCard, NInfiniteScroll } from 'naive-ui';
import { commitContent, fileDiff } from '@/utils/command';
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

const commitFiles = ref<File[]>()
const showedFiles = ref<File[]>([])

onMounted(async () => {
  commitFiles.value = await commitContent(props.repo, props.commitId)
  
  // 先展示到滚动条出现为止
  await nextTick()
  showedFiles.value = commitFiles.value.slice(0, 10)

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

const handleLoad = () => {
  if (loading.value || noMore.value) return
  loading.value = true
  setTimeout(() => {
    loading.value = false
    if (showedFiles.value.length >= commitFiles.value!.length) {
      noMore.value = true
      return
    }
    showedFiles.value = commitFiles.value!.slice(0, showedFiles.value.length + 10)
  }, 1000)
}

const loading = ref<Boolean>(false)
const noMore = ref<Boolean>(false)

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

// 判断滚动条是否出现滚动条
const isScroll = () => {
  const container = containerRef.value
  if (!container) return false
  return container.querySelector('.n-scrollbar-rail:not(.n-scrollbar-rail--disabled)')
}
</script>

<template>
  <div v-if="__show" 
    class="w-screen h-screen 
    bg-slate-400/50 flex 
    items-center justify-center 
    fixed top-0 
    left-0 z-[3]">
    <NCard title="提交详情" class="w-[80%] h-[80%]" closable @close="close">
      <div class="h-full relative" ref="containerRef">
        <NInfiniteScroll class="absolute" :style="containerStyle" :distance="20" @load="handleLoad">
          <template v-for="item in commitFiles" :key="item.objectId">
            <DiffDetailComponent :repo="repo" :file="item" />
          </template>
        </NInfiniteScroll>
      </div>
    </NCard>
  </div>
</template>


<style scoped>

</style>