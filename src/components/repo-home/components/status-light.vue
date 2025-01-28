<script setup lang="ts">
import { parseStatus, RepoStatus } from '@/enum';
import { NTooltip } from 'naive-ui';
import { computed, PropType, toRaw } from 'vue';

defineOptions({
  name: 'StatusLight'
})

const props = defineProps({
  status: {
    type: Number as PropType<RepoStatus>,
    required: true
  }
})

// 计算样式
const style = computed(() => {
  let s = ''
  const map = new Map([
    [RepoStatus.Modified,   '#95e1d3'],
    [RepoStatus.Untracked,  '#eaffd0'],
    [RepoStatus.Uncommitted,'#fce38a'],
    [RepoStatus.Unpushed,   '#f38181'],
    [RepoStatus.Ok,         '#ffffffaa']
  ])
  const status = parseStatus(toRaw(props.status))
  if (status.length === 0) {
    return ''
  }
  if (status.length === 1) {
    s = map.get(status[0])!
  }
  else {
    s = 'linear-gradient('
    for (let i = 0; i < status.length; i++) {
      const start = i / (status.length) * 100
      const end = (i + 1) / (status.length) * 100
      s += ` ${map.get(status[i])} ${start}% ${end}%`
      if (i !== status.length - 1) {
        s += ','
      }
    }
    s += ')'
  }
  return{
    background: s
  }
})

// 计算提示文本
const tip = computed(() => {
  let s = ''
  const status = parseStatus(toRaw(props.status))
  if (status.length === 0) {
    return ''
  }
  const textMap = new Map([
    [RepoStatus.Modified,   '修改'],
    [RepoStatus.Untracked,  '新增'],
    [RepoStatus.Uncommitted,'未提交'],
    [RepoStatus.Unpushed,   '未推送的提交']
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