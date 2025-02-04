<script setup lang="ts">
import { Commit } from '@/types';
import dayjs from 'dayjs';
import { NCard, NEllipsis } from 'naive-ui';
import { PropType } from 'vue';
const props = defineProps({
  commit: {
    type: Object as PropType<Commit>,
    default: ''
  }
})
</script>

<template>
  <NCard hoverable :data-commit-id="commit.commitId">
    <template #header>
      {{commit.title}}
    </template>
    <template #header-extra>
      <span class="text-sm text-gray-500">
        {{commit.authorName}}
      </span>
    </template>
    <div class="flex flex-col">
      <div>{{ commit.message }}</div>
      <div class="text-end">{{ dayjs(commit.datetime).format('YYYY-MM-DD HH:mm:ss') }}</div>
    </div>
  </NCard>
</template>


<style scoped>

</style>