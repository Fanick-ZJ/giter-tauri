<script setup lang="ts">
import { Icon } from '@iconify/vue';
import { NFloatButton, NIcon } from 'naive-ui';

const emit = defineEmits(['shrink']);

</script>
<template>
<div class="group h-full 
            w-[10px] flex
            justify-center items-center
            cursor-pointer">
  <NFloatButton
    class="z-[3] group-hover:opacity-100 opacity-0 
          transition-opacity duration-300 
          ease-in-out"
    @click="() => emit('shrink')">
    <NIcon>
      <Icon icon="weui:back-filled" width="12" height="24" />
    </NIcon>
  </NFloatButton>
</div>
</template>


<style scoped>

</style>