<script setup lang="ts">
import { useThemeStore } from '@/store/modules/theme';
import { NConfigProvider,
        darkTheme,
        zhCN
      } from 'naive-ui';
import { computed, onMounted } from 'vue';
import AppProvider from './components/common/app-provider.vue';
import { RouterView } from 'vue-router';

const themeStore = useThemeStore()
const naviDarkTheme = computed(() => (themeStore.isDark ? darkTheme : undefined))

onMounted(() => {
  // 禁止右键菜单、滚动条
  document.body.style.overflowX = 'hidden'
  document.addEventListener('contextmenu', (e: any) => {
    e.preventDefault() 
  })
})
</script>

<template>
  <NConfigProvider :theme="naviDarkTheme" :locale="zhCN">
    <AppProvider>
      <RouterView></RouterView>
    </AppProvider>
  </NConfigProvider>
</template>

<style>
* {
  user-select: none;
}
</style>