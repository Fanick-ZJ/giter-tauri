<script setup lang="ts">
import { PropType, ref, useTemplateRef } from 'vue';
import FileTree from './tree.vue';
import { NCard, NButton, NFlex, NInput } from 'naive-ui';
import { DialogStatus, SelectFilter } from './types';
defineOptions({
  name: 'FileSelector'
})

defineProps({
  path: {
    type: String,
    required: false,
  },
  multiple: {
    type: Boolean,
    required: false,
  },
  directory: {
    type: Boolean,
    required: false,
  },
  filters: {
    type: Object as PropType<SelectFilter>,
    required: false,
  },
  root: {
    type: String,
    required: false,
  }
})
// 是否显示
const __show = ref<Boolean>(false)
// 选中的文件路径
const selected = ref<string>('')
// 窗口状态
let status: DialogStatus | null = null
// 关闭回调
let closeCallback: (() => any ) | null = null
const fileTreeRef = useTemplateRef<typeof FileTree>('fileTreeRef')

// 显示窗口, 并返回一个promise, 用于返回选择的文件路径
let resolve: any
let reject: any
let show = () => {
  __show.value = true
  return new Promise((res, rej) => {
    resolve = res
    reject = rej
  })
}

const close = () => {
  // 关闭时, 调用resolve或reject
  if (status === 'close') {
    reject()
  } else {
    resolve(undefined)
  }
  __show.value = false
  if (closeCallback) {
    closeCallback()
  }
}

const ok = () => {
  status = 'success'
  resolve(selected.value)
  close()
}

const changed = (val: string) => {
  selected.value = val
}

defineExpose({
  selected,
  status,
  closeCb: (close: () => any) => {
    closeCallback = close
  },
  show,
  close
})

</script>

<template>
  <div v-if="__show" 
    class="w-screen h-screen bg-slate-400/50
    flex items-center justify-center fixed top-0 left-0">
    <div class="w-[230px]">
      <NCard title="选择文件" size="small" closable @close="close">
        <NInput placeholder="请输入文件路径" type="text" size="tiny" clearable :value="selected"/>
        <FileTree ref="fileTreeRef"
          @change="changed"
        />
        <template #footer>
          <NFlex justify="end">
            <NButton size="tiny" type="primary" @click="ok"> 确定</NButton>
            <NButton size="tiny" type="info" @click="close"> 关闭</NButton>
          </NFlex>
        </template>
      </NCard>
    </div>
  </div>
</template>

<style scoped>

</style>