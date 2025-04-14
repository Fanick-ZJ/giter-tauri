<script setup lang="ts">
import { useRepoStore } from '@/store/modules/repo';
import { computed, onBeforeUnmount, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { Icon } from '@iconify/vue'
import LayoutPage from '@/components/common/layout-page/index.vue'
import { getAuthors, getBranchCommitsAfterFilter, beforeReferenceCommitsCount, getBranches, getCurrentBranch, singleRepoSubmit, reference_commit_filter_count, reference_commit_filter_details } from '@/utils/command';
import { Author, Branch, Commit, CommitFilter, Repository } from '@/types';
import CommitItem from './components/commit-item.vue'
import { NFlex, NPagination, NButton, NIcon, NSelect, NDropdown, NLayout } from 'naive-ui';
import FilterForm from './components/filter-form.vue'
import { useContextMenu } from './hook';
import { listen } from '@tauri-apps/api/event';
import { STATUS_CHANGE, StatusChangePayloadType } from '@/const/listen';
import _ from 'lodash';
import { hasFlag, RepoStatus } from '@/enum';
import { withMinDelay } from '@/utils/tool';

const INIT_PAGE = 1
const INIT_PAGE_SIZE = 10

const route = useRoute()
const repoStore = useRepoStore()
const loading = ref(false)
const id = ref(parseInt(route.params.id as string))
const repo = ref<Repository>()
const page = ref(INIT_PAGE)
const pageSize = ref(INIT_PAGE_SIZE)

// 获取分支列表
const branches = ref<Branch[]>([])
const curBranch = ref<Branch>()
const authors = ref<Author[]>([])
const commits = ref<Commit[]>([])
const commitsCount = ref(0)
// 筛选模型
const filterModel = ref<CommitFilter>({
  lastId: undefined,
  author: undefined,
  startTime: undefined,
  endTime: undefined,
  message: undefined,
})

let single_repo_unlisten
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
    commitsCount.value = await reference_commit_filter_count(path, curBranch.value!.name, filterModel.value)
    getCommits()
  })
}

const filterChanged = _.debounce(async () => {
  let path = repo.value!.path
  const t1 = Date.now()
  commitsCount.value = await reference_commit_filter_count(path, curBranch.value!.name, filterModel.value)
  console.log("统计数量", commitsCount.value)
  const t2 = Date.now()
  console.log("统计数量耗时", t2 - t1)
  getCommits()
}, 1000)

const getCommits = _.debounce(async () => {
  let path = repo.value!.path
  loading.value = true
  // 如果不足500ms，就等待500ms
  withMinDelay(async () => {
    const start = pageSize.value * (page.value - 1)
    const count = pageSize.value
    const res = await reference_commit_filter_details(path, curBranch.value!.name, filterModel.value, start, count)
    commits.value = res
  }, 500, () => loading.value = false)
  
}, 100)

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
  single_repo_unlisten()
})
// 监听路由变化，重新获取数据
watch(()=> route.path, () => {
  if (route.path.startsWith('/commit')) {
    id.value = parseInt(route.params.id as string)
    repo.value = repoStore.getRepoById(id.value)
    single_repo_unlisten && single_repo_unlisten()
    // 重置分页,筛选刷新
    page.value = 1 
    filterModel.value = {
      lastId: undefined,
      author: undefined,
      startTime: undefined,
      endTime: undefined,
      message: undefined
    }
    init().catch((err) => {
      window.$message.error(err.data) 
    })
  }
}, {immediate: true})

const hasFilter = computed(() => {
  return !_.every(_.values(filterModel.value), _.isUndefined)
})

const selectedBranch = computed({
  get() {
    if (curBranch.value) {
      return curBranch.value.reference
    } else {
     return '' 
    }
  },
  set(val) {
    let branch = selectBranch(val)
    if (branch) {
      curBranch.value = branch
      getCommits()
    }
  }
})
const branchOptions = computed(() => {
  return branches.value.map((branch) => {
    return {
      label: branch.name,
      value: branch.reference
    }
  }) 
})

const selectBranch = (reference: string) => {
  let branch = branches.value.find((branch) => {
    return branch.reference === reference 
  })
  return branch
}

const pageParamChanged = () => {
  getCommits()
}

const showFilter = ref(false)
const toggleFilter = () => {
  showFilter.value = !showFilter.value
}

const { 
  menuX,
  menuY,
  showMenu,
  handleContextMenu,
  menuCloseOutside,
  handleSelect,
  options} = useContextMenu()

</script>
<template>
  <LayoutPage title="提交记录" :subtitle="repo?.alias" :loading="loading">
    <template #header-extra>
      <div class="flex gap-x-[10px]">
        <NSelect v-model:value="selectedBranch" clearable :options="branchOptions" placeholder="选择分支" class="w-[200px]">
        </NSelect>
        <NButton :dashed='hasFilter' @click="toggleFilter">
          <NIcon>
            <Icon icon="mdi:filter-outline" width="100" height="100" />
          </NIcon>
        </NButton>
        <NButton :dashed='hasFilter' @click="getCommits">
          <NIcon>
            <Icon icon="mdi:reload" width="24" height="24" />
          </NIcon>
        </NButton>
      </div>
    </template>
    <template #filter-form>
      <FilterForm v-if="showFilter" :author-list="authors" v-bind:model-value="filterModel" @filter="filterChanged"></FilterForm>
    </template>
    <NFlex @contextmenu="handleContextMenu" :data-repo="repo?.path">
      <template v-for="c in commits">
        <CommitItem :commit="c"/>
      </template>
    </NFlex>
    <template #footer>
      <NFlex justify="center">
        <NPagination 
          :item-count="commitsCount" 
          v-model:page="page"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 30]"
          @update:page="pageParamChanged"
          @update:page-size="pageParamChanged"
          show-size-picker>
        </NPagination>
      </NFlex>
    </template>
  </LayoutPage>
  <NDropdown 
    placement="bottom-start"
    trigger="manual"
    :x="menuX"
    :y="menuY"
    :show="showMenu"
    :options="options"
    @clickoutside="menuCloseOutside"
    @select="handleSelect">
  </NDropdown>
</template>


<style scoped>

</style>