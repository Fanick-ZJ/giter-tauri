<script setup lang="ts">
import { useThemeStore } from '@/store/modules/theme';
import { NConfigProvider,
        darkTheme,
      } from 'naive-ui';
import { computed } from 'vue';
import AppProvider from './components/common/app-provider.vue';
import HomePage from '@/components/index.vue'

const themeStore = useThemeStore()

const naviDarkTheme = computed(() => (themeStore.isDark ? darkTheme : undefined))


</script>

<template>
  <NConfigProvider :theme="naviDarkTheme">
    <AppProvider>
      <HomePage/>
    </AppProvider>
  </NConfigProvider>
</template>

<style>
* {
  user-select: none;
}
</style>