<script setup lang="ts">
import { useElementSize } from '@vueuse/core';
import { NScrollbar } from 'naive-ui';
import LoadingView from '@/components/common/loading-view.vue';
import { computed, ref, useTemplateRef } from 'vue';
defineOptions({
  name: 'LayoutPage'
})

const props = defineProps({
  title: {
    type: String,
    default: '' 
  },
  subtitle: {
    type: String,
    default: '' 
  },
  padding: {
    type: Number,
    default: 10
  },
  loading: {
    type: Boolean,
    default: false 
  }
})
const pageRef = ref<HTMLElement>()
const footerRef = ref<HTMLElement>()
const pageSize = useElementSize(pageRef)
const footerSize = useElementSize(footerRef)

const contentHeight = computed(() => {
  const height = pageSize.height.value - 35 - footerSize.height.value  - props.padding * 2
  return height
})

</script>
<template>
  <!-- 头部 -->
  <div class="h-screen flex flex-col gap-[5px]" ref="pageRef" :style="{padding: `${padding}px`}">
    <div class="flex h-[35px] justify-between">
      <div>
        <span class="font-bold text-lg/8 inline-block">{{title}}</span>
        <span class="inline-block ml-5 align-bottom text-gray-500 text-base/8">{{subtitle}}</span>
      </div>
      <div>
        <slot name="header-extra"/>
      </div>
    </div>
    <!-- 筛选表单部分 -->
    <div>
      <slot name="filter-form"/>
    </div>
    <LoadingView :loading="loading">
      <NScrollbar>
        <slot />
      </NScrollbar>
    </LoadingView>
    <div ref="footerRef">
      <slot name="footer"/>
    </div>
  </div>
</template>

<style scoped>

</style>