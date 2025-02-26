<script setup lang="ts">
import { ChangedFile } from '@/types';
import { computed, PropType, ref } from 'vue';
import { Icon } from '@iconify/vue'
import FileIcon from '@/components/common/file-icon/index.vue';
import { basename } from '@/utils/tool';
import { NPopover } from 'naive-ui';

defineOptions({
  name: 'ChangedFileWidget'
})

const props = defineProps({
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

const hover = ref(false)
</script>

<template>
  <div>
    <NPopover trigger="hover" :delay="500" :show-arrow="false">
      <template #trigger>
        <div class="flex items-center">
          <FileIcon :path-or-name="file.path"></FileIcon>
          <div class="flex overflow-hidden min-w-0 gap-1 items-end" @mouseenter="hover=true" @mouseleave="hover=false">
            <span class="text-sm truncate">{{ name }}</span>
            <span class="text-xs truncate text-gray-400">{{ file.path }}</span>
            <div class="w-[40px] flex" v-if="hover">
              <Icon icon="codicon:discard" width="18" height="18" />
              <Icon icon="mingcute:add-line" width="18" height="18" />
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