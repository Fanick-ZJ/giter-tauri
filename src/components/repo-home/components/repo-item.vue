<script setup lang="ts">
import { Repository } from '@/types';
import { NCard, NEllipsis } from 'naive-ui';
import { computed, PropType } from 'vue';
import { RepoStatus } from '@/enum';
import { useRepoStore } from '@/store/modules/repo';

import Glassmorphism from '@/components/common/glassmorphism.vue';
import StatusLight from './status-light.vue';

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

const click = () => {
  if (!props.repo.valid) {
    window.$message.warning('无效仓库')
  }
}
</script>
<template>
  <!-- 若仓库为无效仓库，就添加斜线标志 -->
  <NCard 
    :class="repo.valid ? '' : 'bg-diagonal-stripes bg-repeat bg-stripes shadow-lg'"
    content-style="font-size: 20px"
    @click="click">
    <div class="relative">
      <div class="absolute left-[-15px] h-full">
        <StatusLight :status="status" />
      </div>
      <NEllipsis class="flex-auto">
        <Glassmorphism class="inline-block px-1">
          {{repo.alias}}
        </Glassmorphism>
      </NEllipsis>
    </div>
  </NCard>
</template>


<style scoped lang="scss">
</style>