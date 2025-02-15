<script setup lang="ts">import { computed, ref, toRaw, watch } from 'vue';
import { useRoute } from 'vue-router';
import { NLayout, NSkeleton, NSpace, NSelect } from 'naive-ui';
import LayoutPage from '@/components/common/layout-page/index.vue'
import { useRepoStore } from '@/store/modules/repo';
import { Author, Branch, CommitStatistic, Repository, YMDStr } from '@/types';
import { getAuthors, getBranchCommitContribution, getBranches, getCurrentBranch, getGlobalAuthor, getRepoAuthor } from '@/utils/command';
import HashAvatar from '@/components/common/hash-avatar/index.vue'
import CommitHot from './components/commit-hot.vue'
import { emptyAuthor } from '@/types/util';
import _ from 'lodash'
import { DayStat } from './types';

const route = useRoute()
const repoStore = useRepoStore()
const repo = ref<Repository>()
const branches = ref<Branch[]>([])
const currentBranch = ref<Branch>()
const contribution = ref<CommitStatistic[]>([])
const curAuthor = ref<Author>()
const authors = ref<Author[]>([])
const globalAuthor = ref<Author>()

const loading = ref(true)

const init = async () => {
  loading.value = true
  let path = repo.value!.path
  try{
    currentBranch.value = await getCurrentBranch(path)
  }
  catch(err) {
    window.$message.error('获取当前分支失败')
    return
  }
  const _getBranches = getBranches(path)
  const _getAuthor = getRepoAuthor(repo.value!.path)
  const _getGlobalAuthor = getGlobalAuthor()
  const _getAuthors = getAuthors(path, currentBranch.value!)
  const _getContribution = getBranchCommitContribution(path, currentBranch.value!)
  Promise.allSettled([_getBranches, _getContribution, _getAuthor, _getGlobalAuthor, _getAuthors]).then((res) => {
    if (res[0].status === 'rejected') {
      window.$message.error('获取分支失败')
      return
    }
    branches.value = res[0].value 
    if (res[1].status ==='rejected') {
      window.$message.error('获取贡献失败')
      return
    }
    contribution.value = res[1].value
    if (res[4].status ==='rejected') {
      window.$message.error('获取作者失败')
      return 
    }
    authors.value = res[4].value
    let repoAuthor;
    if (res[2].status === 'fulfilled') {
      repoAuthor = res[2].value
    }
    let globalAuthor;
    if (res[3].status === 'fulfilled') {
      globalAuthor = res[3].value
    }
    // 设置默认显示的作者
    // 显示顺序为：仓库作者，全局作者，第一个作者
    let email = repoAuthor?.email || globalAuthor?.email || authors.value[0].email
    curAuthor.value = authors.value.find((author) => {
      return author.email === email
    }) || authors.value[0]
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

const AUTHOR_SPEARATER = '%SPEARATER%'
const authorKey = (author: Author) => {
  return author.email + AUTHOR_SPEARATER + author.name
}

const selectedAuthor = computed({
  get() {
    return curAuthor.value && authorKey(curAuthor.value) || ''
  }, 
  set(val) {
    curAuthor.value = authors.value.find((author) => {
      return authorKey(author)  === val
    }) 
  }
})

const authorOptions = computed(() => {
  return authors.value.map((author) => {
    return {
      label: author.name,
      value: authorKey(author)
    }
  })
})

const selectedContribution = computed(() => {
  let filted = contribution.value.filter((stat) => {
    return stat.author.email === curAuthor.value?.email || stat.author.name === curAuthor.value?.name
  })
  if (filted.length === 0) {
    return [] 
  }
  const allStats = toRaw(filted[0].stats)
  let dates = Object.keys(allStats).sort()
  let years = _.range(parseInt(dates[0].split('-')[0]), parseInt(dates[dates.length - 1].split('-')[0]) + 1)
  let i = 0
  let yearStats = years.map((year) => {
    let stats: DayStat[] = []
    for(; i < dates.length; i++) {
      if(dates[i].startsWith(year.toString())) {
        stats.push({
          date: dates[i] as YMDStr,
          count: allStats[dates[i]]
        }) 
      } else {
        break 
      }
    }
    return {
      year,
      stats
    }
  })
  return yearStats
})

const handleSwitchYear = (year: number) => {
 console.log(year) 
}

const handleClick = (date: string) => {
  console.log(date)
}

</script>

<template>
  <LayoutPage title="贡献统计" :subtitle="repo?.alias">
    <NLayout>
      <NSpace vertical v-if="loading">
        <NSkeleton height="40px" width="33%"/>
        <NSkeleton height="40px" width="66%"/>
        <NSkeleton height="40px" width="99%"/>
        <NSkeleton height="40px" width="33%"/>
        <NSkeleton height="40px" width="67%"/>
        <NSkeleton height="40px" width="24%"/>
        <NSkeleton height="40px" width="65%"/>
        <NSkeleton height="20px" width="95%"/>
        <NSkeleton height="50px" width="74%"/>
        <NSkeleton height="80px" width="82%"/>
      </NSpace>
      <div v-else>
        <div class="w-full h-[300px] ">
          <div class="flex relative" v-if="curAuthor || globalAuthor">
            <HashAvatar :author="curAuthor || globalAuthor || emptyAuthor" :width="80" :borderRadius="15"/>
            <div class="flex flex-col ml-5">
              <div class="text-4xl font-bold">{{curAuthor?.name || globalAuthor?.name}}</div>
              <div class="text-xl text-slate-500">{{curAuthor?.email || globalAuthor?.email}}</div>
            </div>
            <NSelect 
              class="w-[200px] absolute right-0 top-0"
              placeholder="选择作者"
              v-model:value="selectedAuthor" :options="authorOptions"/>
          </div>
          <CommitHot @date-click="handleClick" @switch-year="handleSwitchYear" :stats="selectedContribution"/>
        </div>
      </div>
    </NLayout>
  </LayoutPage>
</template>

<style scoped>

</style>