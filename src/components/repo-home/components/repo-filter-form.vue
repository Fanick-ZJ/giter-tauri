<script setup lang="ts">
import { NForm, NFormItem, NInput, NInputNumber, NSwitch, NButton, NRadioGroup, NRadioButton } from 'naive-ui';
import { FilterModel } from '../types';

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
      <NRadioGroup v-model:value="model.hasWatched">
        <NRadioButton key="yes" value="yes">是</NRadioButton>
        <NRadioButton key="no" value="no">否</NRadioButton>
        <NRadioButton key="" value="">不生效</NRadioButton>
      </NRadioGroup>
    </NFormItem>
    <NFormItem label="是否置顶">
      <NRadioGroup v-model:value="model.top">
        <NRadioButton key="yes" value="yes">是</NRadioButton>
        <NRadioButton key="no" value="no">否</NRadioButton>
        <NRadioButton key="" value="">不生效</NRadioButton>
      </NRadioGroup>
    </NFormItem>
    <NFormItem label="是否有效">
      <NRadioGroup v-model:value="model.valid">
        <NRadioButton key="yes" value="yes">是</NRadioButton>
        <NRadioButton key="no" value="no">否</NRadioButton>
        <NRadioButton key="" value="">不生效</NRadioButton>
      </NRadioGroup>
    </NFormItem>
    <NFormItem>
      <NButton type="primary" @click="handleClick">查询</NButton>
    </NFormItem>
  </NForm>
</template>

<style lang="scss" scoped>

</style>