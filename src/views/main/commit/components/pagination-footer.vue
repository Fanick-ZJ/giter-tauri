<script setup lang="ts">
import { NPagination, NFlex } from 'naive-ui'

defineProps<{
  page: number
  pageSize: number
  total: number
  loading: boolean
}>()

const emit = defineEmits(['update:page', 'update:page-size'])
</script>

<template>
  <NFlex justify="center">
    <NPagination
      :disabled="loading"
      :item-count="total"
      :page="page"
      :page-size="pageSize"
      :page-sizes="[10, 20, 30]"
      @update:page="val => emit('update:page', val)"
      @update:page-size="val => emit('update:page-size', val)"
      show-size-picker>
    </NPagination>
  </NFlex>
</template>
