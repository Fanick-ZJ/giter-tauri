<script setup lang="ts">
import { useThemeStore } from '@/store/modules/theme';
import { NConfigProvider,
        darkTheme,
        NFlex
      } from 'naive-ui';
import { computed, nextTick, onMounted, provide, ref, useTemplateRef, watch } from 'vue';
import AppProvider from './components/common/app-provider.vue';
import RepoHome from '@/components/repo-home/index.vue'
import { viewExtend, viewShrink } from './types/key';
import ExtendPage from '@/components/extend-page/index.vue'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

const themeStore = useThemeStore()
const homeRef = ref<InstanceType<typeof RepoHome>>()

const naviDarkTheme = computed(() => (themeStore.isDark ? darkTheme : undefined))

onMounted(() => {
 // 取消右键事件
 homeRef.value?.$el.addEventListener('contextmenu', (e: any) => {
    e.preventDefault()  
  }) 
})

const homPageRef = useTemplateRef('homeRef')

const _isExtend = ref(false)
const viewToExtend = async () => {
  if (_isExtend.value) return
  _isExtend.value = true
  await nextTick()
  const width = homeRef.value?.$el.clientWidth
  getCurrentWindow().setSize(new LogicalSize(width! + 300, 700))
}

const viewToShrink = () => {
  _isExtend.value = false
  const width = homeRef.value?.$el.clientWidth
  getCurrentWindow().setSize(new LogicalSize(width!, 700))
}

provide(viewExtend, viewToExtend)
provide(viewShrink, viewToShrink)

const homeStyle = computed(() => {
  const width = homeRef.value?.$el.clientWidth
  return {
    width: _isExtend.value? width + 'px' : '100%'
  } 
})
</script>

<template>
  <NConfigProvider :theme="naviDarkTheme">
    <AppProvider>
      <NFlex>
        <RepoHome ref="homeRef" :style="homeStyle"/>
        <div class="flex-1" v-show="_isExtend">
          <ExtendPage/>
        </div>
      </NFlex>
    </AppProvider>
  </NConfigProvider>
</template>

<style>
* {
  user-select: none;
}
</style>