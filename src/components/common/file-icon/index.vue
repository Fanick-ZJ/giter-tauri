<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import { fileExtensionIconMap, fileIcons, fileNameIconMap } from './fileicons';

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
  const basename = props.pathOrName.split('/').pop()?.toLocaleLowerCase() || ''
  const ext = basename.split('.').pop()?.toLocaleLowerCase() || ''
  let svg = ''
  if (fileNameIconMap.has(basename)) {
    svg = fileNameIconMap.get(basename)!
  }
  else if (fileExtensionIconMap.has(ext)) {
    svg = fileExtensionIconMap.get(ext)!
  }
  else svg = fileIcons.defaultIcon.name
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