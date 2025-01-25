<script setup lang="ts">
import { NLayout, NLayoutHeader } from 'naive-ui';

import RepoList from './components/repo-list.vue'
import HomePageHeaders from './components/header.vue'

defineOptions({
  name: 'RepoHome'
})

</script>

<template>
<div class="h-screen relative">
  <NLayout 
  class="h-screen" 
  content-style="padding: 10px"
  position="absolute"
  :native-scrollbar="false">
  <NLayoutHeader class="mb-[5px] h-[35px]" bordered >
    <HomePageHeaders/>
  </NLayoutHeader>
  <NLayout 
    position="absolute"
    style="top: 45px"
    content-style="padding: 10px"
    :native-scrollbar="false">
    <RepoList />
  </NLayout>
</NLayout>
</div>
</template>

<style lang="scss" scoped>

</style>