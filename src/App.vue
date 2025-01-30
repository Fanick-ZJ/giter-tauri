<script setup lang="ts">
import { useThemeStore } from '@/store/modules/theme';
import { NConfigProvider,
        darkTheme,
      } from 'naive-ui';
import { computed, onMounted, ref } from 'vue';
import AppProvider from './components/common/app-provider.vue';
import HomePage from '@/components/index.vue'

const themeStore = useThemeStore()
const homeRef = ref<InstanceType<typeof HomePage>>()

const naviDarkTheme = computed(() => (themeStore.isDark ? darkTheme : undefined))

onMounted(() => {
 // 取消右键事件
 homeRef.value?.$el.addEventListener('contextmenu', (e: any) => {
    e.preventDefault()  
  }) 
})

</script>

<template>
  <NConfigProvider :theme="naviDarkTheme">
    <AppProvider>
      <HomePage ref="homeRef"/>
    </AppProvider>
  </NConfigProvider>
</template>

<style>
* {
  user-select: none;
}
</style>