<script setup lang="ts">
import LayoutPage from '@/components/common/layout-page/index.vue'
import { useRepoStore } from '@/store/modules/repo';
import { Author, Branch, CommitStatistic, Repository } from '@/types';
import { getBranchCommitContribution, getBranches, getCurrentBranch, getGlobalAuthor, getRepoAuthor } from '@/utils/command';
import { ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { NLayout, NSkeleton, NSpace } from 'naive-ui';
import { listen } from '@tauri-apps/api/event';
import { BRANCH_COMMIT_CONTRIBUTION_KEY } from '@/const/listen';

const route = useRoute()
const repoStore = useRepoStore()
const repo = ref<Repository>()
const branches = ref<Branch[]>([])
const currentBranch = ref<Branch>()
const contribution = ref<CommitStatistic[]>([])
const curAuthor = ref<Author>()

const loading = ref(false)

const listen_branch_commit_contribution = (key: string) => {
}

const init = async () => {
  loading.value = true
  let path = repo.value!.path
  try{
    currentBranch.value = await getCurrentBranch(path)
  }
  catch(err) {
    window.$message.error('获取当前分支失败') 
  }
  let _getBranches = getBranches(path)
  let key = Date.now().toString()
  let _getAuthor = getRepoAuthor(repo.value!.path)
  let _getGlobalAuthor = getGlobalAuthor()
  let _getContribution = getBranchCommitContribution(path, currentBranch.value!)
  Promise.all([_getBranches, _getContribution, _getAuthor, _getGlobalAuthor]).then((res) => {
    branches.value = res[0]
    contribution.value = res[1]
    console.log(contribution.value)
    curAuthor.value = res[2] 
  }).finally(() => {
    loading.value = false 
  })
}

// 监听路由变化，重新获取数据
watch(()=> route.path, () => {
  if (route.path.startsWith('/contribution')) {
    repo.value = repoStore.getRepoByPath(route.params.path as string)
    init().catch((err) => {
      window.$message.error(err) 
    })
  }
}, {immediate: true})

</script>

<template>
  <LayoutPage title="贡献统计" :subtitle="repo?.alias">
    <NLayout>
      <NSpace v-if="loading">
        <NSkeleton height="40px" width="33%"/>
        <NSkeleton height="40px" width="66%"/>
        <NSkeleton height="40px" width="99%"/>
      </NSpace>
      <div v-else>
        加载完毕
      </div>
    </NLayout>
  </LayoutPage>
</template>

<style scoped>

</style>