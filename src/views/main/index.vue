<script setup lang="ts">
import { NFlex, useMessage } from 'naive-ui'
import { EXPAND_MIN_WIDTH, MIN_HEIGHT, REPOLIST_MAX_WIDTH, REPOLIST_WIDTH } from '@/const'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import { nextTick, onMounted, provide, ref, StyleValue } from 'vue'
import RepoHome from '@/components/repo-home/index.vue'
import ExtendPage from '@/components/extend-page/index.vue';
import { viewExtend, viewShrink } from '@/types/key';
import { useRepoStore } from '@/store/modules/repo'

const homeRef = ref<InstanceType<typeof RepoHome>>()
// 在window上挂在一个message对象实例，方便使用
window.$message = useMessage()

const isExtend = ref(false)
const homeStyle = ref<StyleValue>()
const viewToExtend = async () => {
  if (isExtend.value) return
  isExtend.value = true
  homeStyle.value = {
    maxWidth: isExtend.value? REPOLIST_WIDTH + 'px' : '100%',
    minWidth: isExtend.value? REPOLIST_WIDTH + 'px' : ''
  } 
  await nextTick()
  const curWind = getCurrentWindow()
  const {width, height} = await curWind.innerSize()
  curWind.setSize(new LogicalSize(REPOLIST_WIDTH + 770, height))
  curWind.setMinSize(new LogicalSize(EXPAND_MIN_WIDTH, MIN_HEIGHT))
  curWind.setMaxSize(null)
  
}

const viewToShrink = () => {
  isExtend.value = false
  const height = homeRef.value?.$el.clientHeight
  homeStyle.value = {
    maxWidth: isExtend.value? REPOLIST_MAX_WIDTH + 'px' : '100%',
    minWidth: isExtend.value? REPOLIST_WIDTH + 'px' : ''
  } 
  const curWind = getCurrentWindow()
  curWind.setMaxSize(new LogicalSize(REPOLIST_MAX_WIDTH, 99999999))
  curWind.setSize(new LogicalSize(REPOLIST_WIDTH, height!))
  curWind.setMinSize(new LogicalSize(REPOLIST_WIDTH, MIN_HEIGHT))
}

provide(viewExtend, viewToExtend)
provide(viewShrink, viewToShrink)

useRepoStore().init_repo()
</script>

<template>
  <div>
    <NFlex :wrap="false" @contextmenu.prevent>
        <RepoHome ref="homeRef" :style="homeStyle"/>
        <div class="flex-1" v-show="isExtend">
          <ExtendPage/>
        </div>
      </NFlex>
  </div>
</template>