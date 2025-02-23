<script setup lang="ts">
import { PropType } from 'vue';
import { ChangedFiles } from '../types';

const prop = defineProps({
  file: {
    type: Object as PropType<ChangedFiles>,
    required: true
  },
  type: {
    type: String as PropType<'changed' | 'staged'>,
  }
})
</script>

<template>
  <div>

  </div>
</template>

<style scoped>

</style>