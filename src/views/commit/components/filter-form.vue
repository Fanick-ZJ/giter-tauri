<script setup lang="ts">
import { Author } from '@/types';
import { NForm, NSelect, NInput, NFormItem, NDatePicker, NButton } from 'naive-ui';
import { computed, PropType } from 'vue';
import { CommitFilter} from '@/types';
import { cp } from 'fs';

const model = defineModel<Omit<Omit<CommitFilter, 'start'>, 'count'>>({
  default: {
    lastId: undefined,
    author: undefined,
    startTime: undefined,
    endTime: undefined,
    message: undefined,
  }
})
const props = defineProps({
  authorList: {
    type: Array as PropType<Author[]>,
    default: () => [] 
  }
})

const timeRange = computed({
  get: () => {
    if (model.value.startTime === undefined || model.value.endTime === undefined) {
      return undefined
    }
    return [model.value.startTime, model.value.endTime] as [number, number]
  },
  set: (timeRange: [number, number] | null) => {
    if (timeRange) {
      model.value.startTime = timeRange[0]
      model.value.endTime = timeRange[1]
    } else {
      model.value.startTime = undefined
      model.value.endTime = undefined
    }
  }
})

const authorSelected = computed({
  get: () => {
    return model.value.author?.email
  },
  set: (email: string) => {
    props.authorList.forEach(author => {
      if (author.email == email) {
        model.value.author = author
      }
    })
  }
})

const authorOptions = computed(() => {
  return props.authorList.map((author) => {
    return {
      label: `${author.name} ${author.email}`,
      value: author.email
    }
  })
})

const clear = () => {
  model.value.lastId = undefined
  model.value.author = undefined
  model.value.startTime = undefined
  model.value.endTime = undefined
  model.value.message = undefined
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
        v-model:value="authorSelected" 
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
      <NInput v-model:value="model.message" size="small"/>
    </NFormItem>
    <NFormItem 
      label="某次提交之前" 
      path="lastId"
      class="h-[30px]">
      <NInput v-model:value="model.lastId" size="small" aria-placeholder="请填写提交ID，至少6位"/>
    </NFormItem>
    <NFormItem 
      label="时间范围" 
      path="content"
      class="h-[30px]">
      <NDatePicker type="datetimerange" clearable  v-model:value="timeRange" size="small"/>
    </NFormItem>
  </NForm>
  <div class="flex justify-end gap-5 mt-1">
    <NButton type="error" @click="clear">重置</NButton>
  </div>
</template>

<style scoped>

</style>