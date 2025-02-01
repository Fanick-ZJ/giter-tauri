<script setup lang="ts">
import { Repository } from '@/types';
import { getRepositoryById } from '@/utils/store';
import { getDirName } from '@/utils/tool';
import { NForm, NInput, NFormItem, NSwitch, useMessage, NInputNumber } from 'naive-ui';
import { computed, onMounted, PropType, ref } from 'vue';

defineOptions({
  name: 'RepoInfoEditForm'
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
    default: -1
  },
  mode: {
    type: String as PropType<'edit' | 'add'>,
    required: true,
  }
})

// 根据props中的id或path获取仓库信息
const getPropRepository = async () => {
  let model: Repository | undefined
  if (props.id) {
    // 通过id获取仓库信息
    model = await getRepositoryById(props.id)
  } else {
    // 通过path获取仓库信息
    model = await getRepositoryById(props.id)
  }
  return model
}

onMounted(async () => {
  if (!props.id && !props.path) {
    console.error('id or path is required')
    editable.value = false
  }
  const _model = await getPropRepository()
  if (_model) {
    model.value = _model
  } else {
    if (props.path) {
      model.value = {
        id: 0,
        path: props.path,
        alias: '',
        top: false,
        hasWatch: true,
        order: 0
      }
    } else {
      editable.value = false
      console.error('repository not found')
      useMessage().error('仓库不存在')
    }
  }
})

const model = ref<Repository>({
  id: 0,
  path: '',
  alias: '',
  top: false,
  hasWatch: false,
  order: 0
})
const editable = ref<Boolean>(true)
const defaultName = computed(() => {
  return props.path ? getDirName(props.path) : ''
})

const aliasDoubleClick = () => {
  if (!model.value.alias) {
    model.value.alias = defaultName.value as string
  }
}

const getModel = () => {
  if (model.value.alias === '') {
    model.value.alias = defaultName.value as string
  }
  return model.value
}

defineExpose({
  getModel
})
</script>

<template>
  <NForm
  :model="model"
  label-placement="left"
  label-width="auto"
  :size="'small'">
    <NFormItem label="仓库路径" path="path">
      <NInput v-model:value="model.path" :disabled="!editable"/>
    </NFormItem>
    <NFormItem label="简称" path="alias">
      <NInput v-model:value="model.alias" :disabled="!editable" :placeholder="defaultName" @click="aliasDoubleClick"/>
    </NFormItem>
    <NFormItem label="置顶" path="top">
      <NSwitch v-model:value="model.top" :disabled="!editable"/>
    </NFormItem>
    <NFormItem label="实时状态" path="hasWatch">
      <NSwitch v-model:value="model.hasWatch" :disabled="!editable"/>
    </NFormItem>
    <NFormItem label="顺序" path="order">
      <NInputNumber v-model:value="model.order" :disabled="!editable" placeholder="数字越小越靠前" :min="0"/>
    </NFormItem>
  </NForm>
</template>

<style scoped>

</style>