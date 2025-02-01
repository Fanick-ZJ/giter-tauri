<script setup lang="ts">
import { useElementSize } from '@vueuse/core';
import { NLayout, NLayoutHeader, NLayoutContent, NLayoutFooter } from 'naive-ui';
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
  }
})
const pageRef = ref<HTMLElement>()
const footerRef = ref<HTMLElement>()
const pageSize = useElementSize(pageRef)
const footerSize = useElementSize(footerRef)

const contentStyle = computed(() => {
  return {
    height: pageSize.height.value - 35 - footerSize.height.value +'px'
  }
})

</script>
<template>
  <div class="h-screen flex flex-col gap-[5px]" ref="pageRef" :style="{padding: `${padding}px`}">
    <div class="flex h-[35px]">
      <span class="font-bold text-lg inline-block">{{title}}</span>
      <span class="inline-block ml-5 align-bottom text-gray-500 text-base/8">{{subtitle}}</span>
    </div>
    <NLayout :style="contentStyle" :native-scrollbar="false">
      <slot/>
    </NLayout>
    <div ref="footerRef">
      <slot name="footer"/>
    </div>
  </div>
</template>

<style scoped>

</style>