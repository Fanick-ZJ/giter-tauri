<script setup lang="ts">
import { Repository } from '@/types';
import { NCard, NEllipsis } from 'naive-ui';
import { computed, PropType } from 'vue';
import StatusLight from './status-light.vue';
import { RepoStatus } from '@/enum';
import { useRepoStore } from '@/store/modules/repo';

const props = defineProps({
  repo: {
    type: Object as PropType<Repository>,
    required: true
  }
})
const store = useRepoStore()
const status = computed(() => {
  return store.status.get(props.repo.path)?.value || RepoStatus.Ok
})
</script>
<template>
  <NCard content-style="font-size: 20px">
    <div class="relative">
      <div class="absolute left-[-15px] h-full">
        <StatusLight :status="status" />
      </div>
      <NEllipsis class="flex-auto">
        {{repo.alias}}
      </NEllipsis>
    </div>
  </NCard>
</template>


<style scoped>

</style>