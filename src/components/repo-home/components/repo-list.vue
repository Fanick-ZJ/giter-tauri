<script setup lang="ts">
import { useRepoStore } from '@/store/modules/repo';
import RepoItem from './repo-item.vue'
import { NFlex } from 'naive-ui';

defineOptions({
  name: 'RepoList'
})

const repoStore = useRepoStore()
</script>

<template>
  <NFlex vertical>
    <template v-for="item in repoStore.repos">
      <RepoItem :repo="item" />
    </template>
  </NFlex>
</template>



<style scoped>

</style>