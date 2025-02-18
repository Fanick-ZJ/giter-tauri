<script setup lang="ts">
import { NForm, NFormItem, NInput, NInputNumber, NSwitch, NButton } from 'naive-ui';
import { FilterModel } from '../types';
import { PropType } from 'vue';

const model = defineModel<FilterModel>({
  default: () => {
    return {
      group: '',
      alias: '',
      path: '',
      order: 0,
      hasWatched: false,
      top: false,
      valid: true
    } 
  }
})

const handleClick = () => {
  emit('filter', model.value) 
}

const emit = defineEmits({
  'update:modelValue': (model: FilterModel) => true,
  'filter': (model: FilterModel) => true
})
</script>

<template>
  <NForm
    :model-value="model"
    label-placement="left"
    label-width="auto"
    :size="'small'"
    >
    <NFormItem label="仓库群组">
      <NInput clearable v-model:value="model.group" />
    </NFormItem>
    <NFormItem label="仓库别名">
      <NInput clearable v-model:value="model.alias" />
    </NFormItem>
    <NFormItem label="仓库地址">
      <NInput clearable v-model:value="model.path" />
    </NFormItem>
    <NFormItem label="序号">
      <NInputNumber clearable v-model:value="model.order"/>
    </NFormItem>
    <NFormItem label="是否监控">
      <NSwitch v-model:value="model.hasWatched"/>
    </NFormItem>
    <NFormItem label="是否置顶">
      <NSwitch v-model:value="model.top"/>
    </NFormItem>
    <NFormItem label="是否有效">
      <NSwitch v-model:value="model.valid"/>
    </NFormItem>
    <NFormItem>
      <NButton type="primary" @click="handleClick">查询</NButton>
    </NFormItem>
  </NForm>
</template>

<style lang="scss" scoped>

</style>