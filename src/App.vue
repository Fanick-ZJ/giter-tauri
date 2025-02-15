<script setup lang="ts">
import { useThemeStore } from '@/store/modules/theme';
import { NConfigProvider,
        darkTheme,
        NFlex,
        zhCN
      } from 'naive-ui';
import { computed, nextTick, onMounted, provide, ref, StyleHTMLAttributes, StyleValue, useTemplateRef, watch } from 'vue';
import AppProvider from './components/common/app-provider.vue';
import RepoHome from '@/components/repo-home/index.vue'
import { viewExtend, viewShrink } from './types/key';
import ExtendPage from '@/components/extend-page/index.vue'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

const themeStore = useThemeStore()
const homeRef = ref<InstanceType<typeof RepoHome>>()

const naviDarkTheme = computed(() => (themeStore.isDark ? darkTheme : undefined))

onMounted(() => {
  document.addEventListener('contextmenu', (e: any) => {
    e.preventDefault() 
  })
})


const isExtend = ref(false)
const homeStyle = ref<StyleValue>()
const viewToExtend = async () => {
  if (isExtend.value) return
  isExtend.value = true
  const homeWidth = homeRef.value?.$el.clientWidth
  homeStyle.value = {
    maxWidth: isExtend.value? homeWidth + 'px' : '100%',
    minWidth: isExtend.value? homeWidth + 'px' : ''
  } 
  await nextTick()
  const curWind = getCurrentWindow()
  const {width, height} = await curWind.innerSize()
  curWind.setSize(new LogicalSize(width + 550, height))
}

const viewToShrink = () => {
  isExtend.value = false
  const width = homeRef.value?.$el.clientWidth
  const height = homeRef.value?.$el.clientHeight
  
  getCurrentWindow().setSize(new LogicalSize(width!, height!))
}

provide(viewExtend, viewToExtend)
provide(viewShrink, viewToShrink)

</script>

<template>
  <NConfigProvider :theme="naviDarkTheme" :locale="zhCN">
    <AppProvider>
      <NFlex :wrap="false" @contextmenu.prevent>
        <RepoHome ref="homeRef" :style="homeStyle"/>
        <div class="flex-1" v-show="isExtend">
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