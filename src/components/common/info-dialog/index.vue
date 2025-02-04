<script setup lang="ts">
import { NButton, NCard, NFlex } from 'naive-ui';
import { onMounted, PropType, ref, toRaw, useTemplateRef } from 'vue';
import RepoInfoEditForm from './form.vue';
import { getFolders } from '@/utils/command';
defineOptions({
  name: 'RepoHome'
})

const props = defineProps({
  path: {
    type: String,
    required: false,
    default: ''
  },
  id: {
    type: Number,
    required: false,
    default: 0
  },
  mode: {
    type: String as PropType<'add' | 'edit'>,
    required: true
  }
})

const formRef = useTemplateRef<typeof RepoInfoEditForm>('formRef')

const __show = ref<Boolean>(false)

onMounted(() => {
  getFolders(props.path).then((res) => {
    if (!res) {
      window.$message.error('当前指定路径不是Git仓库') 
      close()
    }  
  })
})

let closeCallback: (() => any ) | null = null
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
  __show.value = false
  if (closeCallback) {
    closeCallback()
  }
  reject()
}

const ok = () => {
  if (!formRef.value) {
    close()
    return
  }
  resolve(toRaw(formRef.value.getModel()))
  close()
}


defineExpose({
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
    flex items-center justify-center fixed top-0 left-0 z-[3]">
    <div class="w-[280px]">
      <NCard title="添加/修改仓库信息" size="small" closable @close="close">
        <RepoInfoEditForm ref="formRef" v-bind="{...props}"/>
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

<style lang="scss" scoped>

</style>