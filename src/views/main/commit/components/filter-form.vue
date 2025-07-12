<script setup lang="ts">
import { Author } from '@/types';
import { NForm, NSelect, NInput, NButtonGroup, NDatePicker, NButton, NGrid, NFormItemGi } from 'naive-ui';
import { computed, PropType, watch } from 'vue';
import { CommitFilter} from '@/types';
import _ from 'lodash'

const AUTHOR_EMAIL_INTERVAL = "<author-email-interval>"

const model = defineModel<CommitFilter>({
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
  },
  disabled: {
    type: Boolean,
    default: true 
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
    return model.value.author?.email || null
  },
  set: (key: string) => {
    if (!key) {
      model.value.author = undefined
      return
    }
    props.authorList.forEach(author => {
      const [email, name] = key.split(AUTHOR_EMAIL_INTERVAL)
      if (author.email == email && author.name == name) {
        model.value.author = author
        return
      }
    })
  }
})

const authorOptions = computed(() => {
  return props.authorList.map((author) => {
    return {
      label: author.name,
      value: author.email + AUTHOR_EMAIL_INTERVAL + author.name,
    }
  })
})

const clear = () => {
  // 如果model.value的属性都是undefined的话，就不用刷新
  if (_.every(model.value, (item) => item === undefined)) {
    return
  }

  model.value.lastId = undefined
  model.value.author = undefined
  model.value.startTime = undefined
  model.value.endTime = undefined
  model.value.message = undefined
  emit('filter', model.value)
}

const search = () => {
  emit('filter', model.value)
}

const emit = defineEmits(['filter', 'clear'])
</script>
<template>
  <NForm 
    :model="model"
    label-placement="left"
    :disabled="props.disabled">
    <NGrid :cols="24" :x-gap="24">
      <NFormItemGi label="作者" path="author" :span="12">
        <NSelect 
          v-model:value="authorSelected"
          :options="authorOptions" 
          clearable
          size="small"
          placeholder="请选择"/>
      </NFormItemGi>
      <NFormItemGi label="内容" path="content" :span="12">
        <NInput v-model:value="model.message" size="small"/>
      </NFormItemGi>
      <NFormItemGi 
        label="某次提交之前" 
        path="lastId"
        :span="12">
        <NInput v-model:value="model.lastId" size="small" aria-placeholder="请填写提交ID，至少6位"/>
      </NFormItemGi>
      <NFormItemGi label="时间范围" path="content" :span="12">
        <NDatePicker type="datetimerange" clearable  v-model:value="timeRange" size="small"/>
      </NFormItemGi>
    </NGrid>
  </NForm>
  <div class="flex justify-end gap-5">
    <NButton :disabled="props.disabled" type="info" @click="search">查询</NButton>
    <NButton :disabled="props.disabled" type="error" @click="clear">重置</NButton>
  </div>
</template>

<style scoped>

</style>