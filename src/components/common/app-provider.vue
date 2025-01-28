<script setup lang="ts">
import { NDialogProvider, NMessageProvider, NNotificationProvider } from 'naive-ui';

defineOptions({
  name: 'AppProvider'
})
</script>
<template>
<NDialogProvider>
  <NNotificationProvider>
    <NMessageProvider>
    <slot></slot>
  </NMessageProvider>
  </NNotificationProvider>
</NDialogProvider>
</template>
