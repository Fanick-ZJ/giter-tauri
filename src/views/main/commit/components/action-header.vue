<script setup lang="ts">
import { NSelect, NButton, NIcon } from 'naive-ui'
import { Icon } from '@iconify/vue'

defineProps<{
  branch: string  // 修复1：正确定义model属性
  branchOptions: Array<{ label: string; value: string }>
  hasFilter: boolean
}>()

const emit = defineEmits<{
  (e: 'update:branch', value: string): void  // 修复2：正确定义事件类型
  (e: 'refresh'): void
  (e: 'toggle-filter'): void
}>()
</script>

<template>
  <div class="flex gap-x-[10px]">
    <NSelect 
      :value="branch"
      @update:value="val => emit('update:branch', val)"
      :options="branchOptions"
      clearable
      placeholder="选择分支"
      class="w-[200px]">
    </NSelect>
    <NButton :dashed="hasFilter" @click="emit('toggle-filter')">
      <NIcon>
        <Icon icon="mdi:filter-outline" width="20" height="20" />
      </NIcon>
    </NButton>
    <NButton :dashed="hasFilter" @click="emit('refresh')">
      <NIcon>
        <Icon icon="mdi:reload" width="20" height="20" />
      </NIcon>
    </NButton>
  </div>
</template>
