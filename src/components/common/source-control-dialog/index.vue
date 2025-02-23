<script setup lang="ts">
import { onMounted, PropType, ref } from 'vue';
import { NCard, NInput, NButton } from 'naive-ui';
import { ValidRepository } from '@/store/modules/repo';
import { FileStatus } from '@/enum';
import { fileIcons } from '../file-icon/fileicons';
import { getChangedFiles, getStagedFiles } from '@/utils/command';

const props = defineProps({
  repo: {
    type: Object as PropType<ValidRepository>,
    required: true 
  }
})

let _resolve;
let _reject;

const _show = ref(false)
const show = () => {
  _show.value = true
  const promise = new Promise((resolve, reject) => {
    _resolve = resolve
    _reject = reject
  })
return promise
}

const close = () => {
  _show.value = false
  _resolve()
  _closeCb && _closeCb()
}

let _closeCb;

defineExpose({
  show,
  closeCb: (cb) => {
    _closeCb = cb
  }
})

onMounted(() => {
  getStagedFiles(props.repo.path).then((res) => {
    console.log(res) 
  })
})

const commitMsg = ref('')

const changes = ref<FileStatus[]>([])
const staged = ref<FileStatus[]>([])

</script>

<template>
  <div v-if="_show" class="w-screen h-screen bg-slate-400/50 flex items-center justify-center fixed top-0 left-0 z-[3]">
    <NCard class="w-[80%] h-[80%]">
      <template #header>
        <div class="text-base">
          <span>源码控制</span>
        </div>
      </template>
      <NInput maxlength="200" v-model:value="commitMsg" :autosize="{minRows: 1, maxRows: 3}" type="textarea" placeholder="请输入提交内容">
        仓库路径: {{props.repo.path}}
      </NInput>
      <div>
        
      </div>
      <template #footer>
        <div class="flex justify-end">
          <NButton @click="close">
            关闭
          </NButton>
        </div>
      </template>
    </NCard>
  </div>
</template>


<style scoped>

</style>