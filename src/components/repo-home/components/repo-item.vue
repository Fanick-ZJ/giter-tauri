<script setup lang="ts">
import { NCard, NEllipsis, NPopover } from 'naive-ui';
import { computed, inject, nextTick, PropType, ref } from 'vue';
import { RepoStatus } from '@/enum';
import { Icon } from '@iconify/vue';
import { useRepoStore, ValidRepository } from '@/store/modules/repo';

import Glassmorphism from '@/components/common/glassmorphism.vue';
import StatusLight from './status-light.vue';
import { viewExtend } from '@/types/key';
import { useRouter } from 'vue-router';

const props = defineProps({
  repo: {
    type: Object as PropType<ValidRepository>,
    required: true
  }
})
const store = useRepoStore()
const status = computed(() => {
  return store.status.get(props.repo.path)?.value || RepoStatus.Ok
})

const validTip = () => {
  if (!props.repo.valid) {
    window.$message.warning('无效仓库')
    return false
  }
  return true
}

const mouseHover = ref(false)
const onLeave = () => {
  mouseHover.value = false 
}

const _viewExtend = inject(viewExtend)
const router = useRouter()

const toCommit = () => {
  if (!validTip()) return
  router.push({
    name: 'commit',
    params: {
      id: props.repo.id
    } 
  })
  _viewExtend!()
}
</script>
<template>
  <!-- 若仓库为无效仓库，就添加斜线标志 -->
  <NCard 
    :class="repo.valid ? '' : 'bg-diagonal-stripes bg-repeat bg-stripes shadow-lg'"
    content-style="font-size: 20px"
    :data-repo="repo.path"
    class="overflow-hidden">
    <div class="relative">
      <div class="absolute right-[-18px] top-2" v-if="repo.top">
        <NPopover>
          <template #trigger>
            <Icon icon="solar:pin-bold-duotone" width="15" height="15" />
          </template>
          已置顶
        </NPopover>
      </div>
      <div class="absolute left-[-15px] h-full">
        <StatusLight :status="status" />
      </div>
      <NEllipsis class="flex-auto">
        <Glassmorphism class="inline-block px-1">
          {{repo.alias}}
        </Glassmorphism>
      </NEllipsis>
      <!-- 鼠标移入显示 -->
      <div class="absolute right-[calc(var(--n-padding-left)*-1)] top-[-20px] h-[75px] w-[10px]"
        @mouseenter="mouseHover = true">
      </div>
      <Glassmorphism class="
            absolute right-[calc(var(--n-padding-left)*-1)]
            top-[-20px] h-[75px] 
            w-[30px] duration-300
            ease-in flex flex-col items-center justify-center"
        :class="mouseHover ? 'right-[calc(var(--n-padding-left)*-1)]' : 'right-[calc(var(--n-padding-left)*-3)]'"
        @mouseleave="onLeave">
        <div @click.stop='toCommit'><Icon icon="iconoir:git-fork" width="24" height="24" /></div>
        <div><Icon icon="uil:statistics" width="24" height="24" /></div>
      </Glassmorphism>
    </div>
  </NCard>
</template>


<style scoped lang="scss">
</style>