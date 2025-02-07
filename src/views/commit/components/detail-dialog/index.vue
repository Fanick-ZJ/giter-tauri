<script setup lang="ts">
import { computed, nextTick, onBeforeMount, onMounted, StyleValue, useTemplateRef, watch } from 'vue';
import { File } from '@/types';

import { ref } from 'vue';
import { NCard, NFlex, NLayout } from 'naive-ui';
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

onMounted(async () => {
  commitFiles.value = await commitContent(props.repo, props.commitId)
  // 设置滚动条的z-index,在layout上设置了style无效
  containerRef.value!.querySelector('.n-scrollbar-rail')!.setAttribute('style', 'z-index: 4')

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
        <NLayout 
          class="absolute w-full"
          :style="containerStyle" 
          :native-scrollbar="false">
          <NFlex>
            <template v-for="item in commitFiles" :key="item.objectId">
              <DiffDetailComponent :repo="repo" :file="item" />
            </template>
          </NFlex>
        </NLayout>
      </div>
    </NCard>
  </div>
</template>


<style scoped>
</style>