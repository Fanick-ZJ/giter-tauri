<script setup lang="ts">
import { RepoStatus } from '@/enum';
import { NTooltip } from 'naive-ui';
import { computed, PropType, toRaw } from 'vue';

defineOptions({
  name: 'StatusLight'
})

const props = defineProps({
  status: {
    type: Object as PropType<RepoStatus[]>,
    required: true
  }
})

// 计算样式
const style = computed(() => {
  let s = ''
  const map = new Map([
    [RepoStatus.Modified, '#ffde7d'],
    [RepoStatus.Untracked, '#f6416c'],
    [RepoStatus.UnCommitted, '#f6416c'],
    [RepoStatus.Unpushed, '#00b8a9'],
    [RepoStatus.Ok, '#ffffffaa']
  ])
  const status = toRaw(props.status).sort()
  if (status.length === 0) {
    return ''
  }

  if (status.length === 1) {
    s = map.get(status[0])!
  }
  else if (status.length === 2) {
    s = `linear-gradient(${map.get(status[0])} 0% 50%, ${map.get(status[1])} 50% 100%)`
  }
  else {
    s = `linear-gradient(${map.get(status[0])} 0% 33%, ${map.get(status[1])} 33% 66%, ${map.get(status[2])} 66% 100%)`
  }
  return{
    background: s
  }
})

// 计算提示文本
const tip = computed(() => {
  let s = '有'
  const status = toRaw(props.status).sort()
  if (status.length === 0) {
    return ''
  }
  const textMap = new Map([
    [RepoStatus.Modified, '修改'],
    [RepoStatus.Untracked, '新增'],
    [RepoStatus.Unpushed, '未推送的提交']
  ])
  if (status.length === 1 && status[0] == RepoStatus.Ok) {
    return undefined
  }
  for (let item of status) {
    s += textMap.get(item)
    if (status.indexOf(item) !== status.length - 1) {
      s += '、'
    }
  }
  return s
})

</script>

<template>
   <NTooltip placement="bottom" trigger="hover" :disabled="!tip">
      <template #trigger>
      <div class="h-full w-[5px] rounded " :style="style">
      </div>
      </template>
      <span> {{ tip }} </span>
    </NTooltip>
</template>

<style scoped>

</style>