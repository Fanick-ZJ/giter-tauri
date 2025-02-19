<script setup lang="ts">
import { Icon } from '@iconify/vue'

defineOptions({
  name: 'LoadingView'
})
const props = defineProps({
  loading: {
    type: Boolean,
    required: true
  } 
})


</script>

<template>
    <slot v-if="loading" name="loading-part">
      <div class="1111111 w-full h-full flex justify-center items-center">
        <Icon icon="eos-icons:bubble-loading" width="24" height="24" />
      </div>
    </slot>
    <slot v-else></slot>
</template>

<style scope>
</style>