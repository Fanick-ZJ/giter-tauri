<script setup lang="ts">
import { Author } from '@/types';
import { NForm, NSelect, NInput, NFormItem, NDatePicker, NButton } from 'naive-ui';
import { computed, PropType } from 'vue';
import { Model } from './type';

const model = defineModel<Model>({
  default: {
    author: null,
    content: null,
    timeRange: null
  }
})
const props = defineProps({
  authorList: {
    type: Array as PropType<Author[]>,
    default: () => [] 
  }
})
const authorOptions = computed(() => {
  return props.authorList.map((author) => {
    return {
      label: author.name,
      value: author.name
    }
  })
})

const clear = () => {
  model.value.author = null
  model.value.content = null
  model.value.timeRange = null
}

const emit = defineEmits(['filter'])
</script>
<template>
  <NForm 
    :model="model"
    label-placement="left"
    class="flex flex-wrap gap-y-[5px] gap-x-[10px]">
    <NFormItem label="作者"
        path="author"
        class="h-[30px]">
      <NSelect 
        v-model:value="model.author" 
        :options="authorOptions" 
        clearable
        size="small" 
        class="w-[200px]"
        placeholder="请选择"/>
    </NFormItem>
    <NFormItem 
      label="内容" 
      path="content"
      class="h-[30px]">
      <NInput v-model:value="model.content" size="small"/>
    </NFormItem>
    <NFormItem 
      label="时间范围" 
      path="content"
      class="h-[30px]">
      <NDatePicker type="datetimerange" clearable  v-model:value="model.timeRange" size="small"/>
    </NFormItem>
  </NForm>
  <div class="flex justify-end gap-5 mt-1">
    <NButton type="error" @click="clear">重置</NButton>
  </div>
</template>

<style scoped>

</style>