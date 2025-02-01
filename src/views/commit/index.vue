<script setup lang="ts">
import { useRepoStore } from '@/store/modules/repo';
import { computed, nextTick, onBeforeMount, ref, watch } from 'vue';
import { onBeforeRouteUpdate, useRoute } from 'vue-router';

import LayoutPage from '@/components/common/layout-page/index.vue'
import { getBranchCommits, getBranches, getCurrentBranch } from '@/utils/command';
import { Branch, Commit, Repository } from '@/types';
import CommitItem from './commit-item.vue'
import { NFlex, NPagination } from 'naive-ui';

const route = useRoute()
const repoStore = useRepoStore()
const id = ref(parseInt(route.params.id as string))
const repo = ref<Repository>()
const page = ref(1)
const pageSize = ref(10)

// 获取分支列表
const branches = ref<Branch[]>([])
const curBranch = ref<Branch>()
const total = ref(0)
const commits = ref<Commit[]>([])
const init = async () => {
  let path = repo.value!.path
  branches.value = await getBranches(path)
  curBranch.value = await getCurrentBranch(path)
  getBranchCommits(path, curBranch.value, 1 << 31).then((res) => {
    commits.value = res
    console.log(res)
    total.value = commits.value.length
  })
}

watch(()=> route.path, () => {
  if (route.path.startsWith('/commit')) {
    id.value = parseInt(route.params.id as string)
    repo.value = repoStore.getRepoById(id.value)
    init().catch((err) => {
      window.$message.error(err) 
    })
  }
}, {immediate: true})


</script>
<template>
  <LayoutPage title="提交记录" :subtitle="repo?.alias">
    <NFlex>
      <template v-for="c in commits.splice((page - 1) * pageSize, page * pageSize)">
        <CommitItem :commit="c"/>
      </template>
    </NFlex>
    <template #footer>
        <NPagination 
          :item-count="total" 
          v-model:page="page"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 30]"
          show-size-picker>
        </NPagination>
    </template>
  </LayoutPage>
</template>


<style scoped>

</style>