<script setup lang="ts">
import { ChangedFile } from '@/types';
import { computed, PropType, ref } from 'vue';
import { Icon } from '@iconify/vue'
import FileIcon from '@/components/common/file-icon/index.vue';
import { basename } from '@/utils/tool';
import { NPopover } from 'naive-ui';
import { FileStatus } from '@/enum';
import { addFileToStage, checkoutFile, removeFileFromStage } from '@/utils/command';
import { ValidRepository } from '@/store/modules/repo';

defineOptions({
  name: 'ChangedFileWidget'
})

const props = defineProps({
  repo: {
    type: Object as PropType<ValidRepository>,
    required: true 
  },
  file: {
    type: Object as PropType<ChangedFile>,
    required: true
  },
  type: {
    type: String as PropType<'changed' | 'staged'>,
  }
})

const name = computed(() => {
  return basename(props.file.path)
})

const handleDiscard = () => {
  if (props.type === 'staged') {
    removeFileFromStage(props.repo.path, props.file.path)
  } else if (props.type === 'changed') {
    checkoutFile(props.repo.path, props.file.path)
  }
}

const handleAdd = () => {
  addFileToStage(props.repo.path, props.file.path) 
}

const hover = ref(false)
</script>

<template>
  <div>
    <NPopover trigger="hover" :delay="500" :show-arrow="false">
      <template #trigger>
        <div class="flex items-center gap-1">
          <FileIcon :path-or-name="file.path" :width="14" :height="14"></FileIcon>
          <div class="flex overflow-hidden min-w-0 gap-1 items-end" @mouseenter="hover=true" @mouseleave="hover=false">
            <span class="text-sm truncate">{{ name }}</span>
            <span class="text-xs truncate text-gray-400">{{ repo.path + '\\' + file.path }}</span>
            <div class="w-[40px] flex" v-if="hover">
              <Icon icon="codicon:discard" width="18" height="18"  @click="handleDiscard"/>
              <Icon icon="mingcute:add-line" width="18" height="18" v-if="type === 'changed'" @click="handleAdd"/>
            </div>
            <div>
              <div class="text-orange-600" v-if="file.status == FileStatus.Modified">M</div>
              <div class="text-green-400" v-if="file.status == FileStatus.Added">A</div>
              <div class="text-red-400" v-if="file.status == FileStatus.Deleted">D</div>
              <div class="text-blue-500" v-if="file.status == FileStatus.Renamed">R</div>
            </div>
          </div>
        </div>
    </template>
    
    {{ file.path }}
  </NPopover>
  </div>
</template>

<style scoped>

</style>