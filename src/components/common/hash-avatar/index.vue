<template>
  <div class="avatar" v-html="avatar" :style="{width: width+'px', height: width + 'px',borderRadius: borderRadius+'px'}">
  </div>
</template>

<script setup lang="ts" generic="T extends Author">
import { Author } from '@/types'
import { generateFromString } from 'generate-avatar';
import { onBeforeMount, onMounted, ref } from 'vue';

defineOptions({
  name: 'HashAvatar' 
})

const { author, width, borderRadius } = defineProps<{
  author: T
  width: number
  borderRadius: number
}>()


const getAvatar = (name: string, email: string, width: Number = 70, height: Number = 70, className?: string) => {
  let item = generateFromString(name + email).replace("width=\"300\"", `width=\"${width}\"`).replace("height=\"300\"", `height=\"${height}\"`)
  if (className){
      item = item.replace('<svg', `<svg class=${className}`)
  }
  return item
}

const avatar = ref<string>()
onMounted(() => {
  avatar.value = getAvatar(author.name, author.email, width, width)
})
</script>

<style scoped lang="scss">
.avatar{
  overflow: hidden;
}
</style>