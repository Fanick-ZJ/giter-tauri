<script setup lang="ts">
import { useRepoStore } from '@/store/modules/repo';
import { computed, onBeforeUnmount, Ref, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import LayoutPage from '@/components/common/layout-page/index.vue'
import { getAuthors, getBranches, getCurrentBranch, reference_commit_filter_count, reference_commit_filter_details } from '@/utils/command';
import { Author, Branch, Commit, CommitFilter, Repository } from '@/types';
import CommitItem from './components/commit-item.vue'
import { NFlex, NDropdown } from 'naive-ui';
import { useContextMenu } from './hook';
import { listen } from '@tauri-apps/api/event';
import { STATUS_CHANGE, StatusChangePayloadType } from '@/const/listen';
import FilterForm from './components/filter-form.vue'
import _ from 'lodash';
import { hasFlag, RepoStatus } from '@/enum';
import { withMinDelay } from '@/utils/tool';
import { reactive, watchEffect } from 'vue'
import ActionHeader from './components/action-header.vue'
import PaginationFooter from './components/pagination-footer.vue'

const INIT_PAGE = 1
const INIT_PAGE_SIZE = 10

const route = useRoute()
const repoStore = useRepoStore()
const loading = ref(false)
const id = ref(parseInt(route.params.id as string))
const repo = ref<Repository>()

const init = async () => {
  loading.value = true
  let path = repo.value!.path
  const branchesPromise = getBranches(path)
  const curBranchPromise = getCurrentBranch(path)
  return Promise.allSettled([branchesPromise, curBranchPromise]).then(async (res) => {
    if (res[0].status === 'fulfilled') {
      branches.value = res[0].value
    } else {
      window.$message.error('获取分支失败')
      return
    }
    if (res[1].status === 'fulfilled') {
      curBranch.value = res[1].value
    } else {
      window.$message.error('获取当前分支失败')
      curBranch.value = branches.value[0]
    }
    authors.value = await getAuthors(repo.value!.path, curBranch.value)
    pagination.total = await reference_commit_filter_count(path, curBranch.value!.name, filterModel.value)
    getCommits()
  })
}

// 文件变更时，重新获取数据
const changed_listen = listen<StatusChangePayloadType>(STATUS_CHANGE, (event) => {
    if (event.payload.path === repo.value!.path && hasFlag(event.payload.status, RepoStatus.Unpushed)) {
      getCommits()
  }
})

onBeforeUnmount(() => {
  changed_listen.then((unlisten) => {
    unlisten()
  }) 
})

const { 
  menuX,
  menuY,
  showMenu,
  handleContextMenu,
  menuCloseOutside,
  handleSelect,
  options
} = useContextMenu()

// 组合式API逻辑
const useBranchData = () => {
  const branches = ref<Branch[]>([])
  const curBranch = ref<Branch>()
  const authors = ref<Author[]>([])
  const commitsCount = ref(0)

  const selectedBranch = computed({
    get: () => curBranch.value?.reference || '',
    set: (val: string) => {
      const branch = branches.value.find(b => b.reference === val)
      if (branch) curBranch.value = branch
    }
  })

  const branchOptions = computed(() => 
    branches.value.map(b => ({ label: b.name, value: b.reference }))
  )

  return { branches, curBranch, authors, selectedBranch, branchOptions, commitsCount }
}

const useCommitData = (repo: Ref<Repository|undefined>) => {
  const commits = ref<Commit[]>([])
  const pagination = reactive({
    page: INIT_PAGE,
    pageSize: INIT_PAGE_SIZE,
    total: 0
  })

  const getCommits = _.debounce(async () => {
    if (!repo.value?.path || !curBranch.value) return
    
    loading.value = true
    try {
      await withMinDelay(async () => {
        const start = pagination.pageSize * (pagination.page - 1)
        const data = await reference_commit_filter_details(
          repo.value!.path,
          curBranch.value!.name,
          filterModel.value,
          start,
          pagination.pageSize
        )
        commits.value = data
        pagination.total = await reference_commit_filter_count(repo.value!.path, curBranch.value!.name, filterModel.value)
      }, 500)
    } finally {
      loading.value = false
    }
  }, 150, { leading: true, trailing: true })

  watch(
    () => [pagination.page, pagination.pageSize],
    () => {
      getCommits()
    },
    { immediate: true }
  )
  return { commits, pagination, getCommits }
}

const useFilter = () => {
  const showFilter = ref(false)
  const filterModel = ref<CommitFilter>({
    lastId: undefined,
    author: undefined,
    startTime: undefined,
    endTime: undefined,
    message: undefined,
  })

  const hasFilter = computed(() => 
    !_.every(_.values(filterModel.value), _.isUndefined)
  )

  const filterChanged = _.debounce(() => {
    pagination.page = 1
    getCommits()
  }, 200)

  return { 
    filterModel, 
    hasFilter, 
    filterChanged,
    showFilter,
    toggleFilter: () => { showFilter.value = !showFilter.value} 
    }
}

// 主逻辑组合
const { branches, curBranch, selectedBranch, branchOptions, authors} = useBranchData()
const { commits, pagination, getCommits } = useCommitData(repo)
const { filterModel, hasFilter, filterChanged, showFilter ,toggleFilter } = useFilter()

// 监听路由变化，重新获取数据
watch(
  [() => route.path, () => route.params.id],
  ([newPath, newId]) => {
    if (newPath.startsWith('/main/commit')) {
      id.value = parseInt(newId as string)
      repo.value = repoStore.getRepoById(id.value)
      
      // 重置状态
      pagination.page = 1
      filterModel.value = _.mapValues(filterModel.value, () => undefined)
      
      // 统一初始化逻辑
      init().catch((err) => {
        window.$message.error(`获取数据失败: ${err.data}`)
      })
    }
  },
  { immediate: true, flush: 'post' }
)
</script>

<template>
  <LayoutPage title="提交记录" :subtitle="repo?.alias" :loading="loading">
    <template #header-extra>
      <ActionHeader 
        v-model:branch="selectedBranch"
        :branch-options="branchOptions"
        :has-filter="hasFilter"
        @refresh="getCommits"
        @toggle-filter="toggleFilter"
      />
    </template>
    <template #filter-form>
      <FilterForm 
        :disabled="loading" 
        v-if="showFilter" 
        :author-list="authors" 
        v-model="filterModel"
        @filter="filterChanged"/>
    </template>
    <NFlex @contextmenu="handleContextMenu" :data-repo="repo?.path">
      <CommitItem 
        v-for="c in commits"
        :key="c.commitId"
        :commit="c"
      />
    </NFlex>
    <NDropdown 
      placement="bottom-start"
      trigger="manual"
      :x="menuX"
      :y="menuY"
      :show="showMenu"
      :options="options"
      :disabled="loading"
      @clickoutside="menuCloseOutside"
      @select="handleSelect">
    </NDropdown>
    <template #footer>
      <PaginationFooter
        v-model:page="pagination.page"
        v-model:page-size="pagination.pageSize"
        :total="pagination.total"
        :loading="loading"
      />
    </template>
  </LayoutPage>
</template>