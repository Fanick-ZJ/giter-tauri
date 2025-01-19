<script setup lang="ts">
import { useThemeStore } from '@/store/modules/theme';
import { NLayout, NButton, NElement, NLayoutHeader, NLayoutContent } from 'naive-ui';
import HomePageHeaders from './components/header.vue'
defineOptions({
  name: 'RepoHome'
})

const themeStore = useThemeStore()
</script>

<template>
<NLayout class="h-screen" content-style="padding: 10px">
  <NLayoutHeader bordered>
    <HomePageHeaders/>
  </NLayoutHeader>
  <NLayoutContent>
    bbbbbbbbb
  </NLayoutContent>
</NLayout>
</template>

<style lang="scss" scoped>

</style>