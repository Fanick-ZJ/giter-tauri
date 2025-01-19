<script setup lang="ts">
import { NDialogProvider, NMessageProvider,  } from 'naive-ui';

defineOptions({
  name: 'AppProvider'
})
</script>
<template>
<NDialogProvider>
  <NMessageProvider>
    <slot></slot>
  </NMessageProvider>
</NDialogProvider>
</template>
