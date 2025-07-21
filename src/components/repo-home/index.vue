<script setup lang="ts">
import { NLayout, NLayoutHeader, useMessage, useNotification } from 'naive-ui';

import RepoList from './components/repo-list.vue'
import HomePageHeaders from './components/header.vue'
import { useTemplateRef } from 'vue';

defineOptions({
  name: 'RepoHome'
})

const headerRef = useTemplateRef('headerRef')
</script>

<template>
<NLayout
  class="h-screen p-[10px]"
  :native-scrollbar="false">
  <NLayoutHeader class="mb-[5px] h-[35px]" bordered >
    <HomePageHeaders ref="headerRef"/>
  </NLayoutHeader>
  <NLayout 
    position="absolute"
    style="top: 45px"
    :native-scrollbar="false">
    <RepoList :filter="headerRef?.filter"/>
  </NLayout>
</NLayout>
</template>

<style lang="scss" scoped>

</style>