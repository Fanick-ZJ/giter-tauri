<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import { fileExtensionIconMap, fileIcons, fileNameIconMap } from './fileicons';
import { basename, extname } from '@/utils/tool';

defineOptions({
  name: 'FileIcon'
})

const props = defineProps({
  pathOrName: {
    type: String,
    default: ''
  },
  width: {
    type: Number,
    default: 24
  },
  height: {
    type: Number,
    default: 24
  },
  style: {
    type: Object,
    default: () => ({})
  }
})


const svg = computed(() => {
  const file_name = basename(props.pathOrName)
  const ext = extname(file_name)
  let svg = ''
  if (ext == undefined) {
    return svg = fileIcons.defaultIcon.name
  }
  else if (fileNameIconMap.has(file_name)) {
    svg = fileNameIconMap.get(file_name)!
  }
  else if (fileExtensionIconMap.has(ext)) {
    svg = fileExtensionIconMap.get(ext)!
  }
  return defineAsyncComponent(() => import(`@/assets/icons/${svg}.svg`))
})

const style = computed(() => {
  return {
    width: `${props.width}px`,
    height: `${props.height}px`,
    ...props.style
  }
})

</script>
<template>
  <div>
    <component :is="svg" :style="style" />
  </div>
</template>


<style scoped>

</style>