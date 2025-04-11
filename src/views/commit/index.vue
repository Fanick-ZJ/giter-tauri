<script setup lang="ts">
import { useRepoStore } from '@/store/modules/repo';
import { computed, onBeforeUnmount, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { Icon } from '@iconify/vue'
import LayoutPage from '@/components/common/layout-page/index.vue'
import { getAuthors, getBranchCommits, getBranches, getCurrentBranch } from '@/utils/command';
import { Author, Branch, Commit, Repository } from '@/types';
import CommitItem from './components/commit-item.vue'
import { NFlex, NPagination, NButton, NIcon, NSelect, NDropdown, NLayout } from 'naive-ui';
import { Model } from './type';
import FilterForm from './components/filter-form.vue'
import { useContextMenu } from './hook';
import { listen } from '@tauri-apps/api/event';
import { STATUS_CHANGE, StatusChangePayloadType } from '@/const/listen';
import _ from 'lodash';
import { hasFlag, RepoStatus } from '@/enum';
import { withMinDelay } from '@/utils/tool';

const route = useRoute()
const repoStore = useRepoStore()
const loading = ref(false)
const id = ref(parseInt(route.params.id as string))
const repo = ref<Repository>()
const page = ref(1)
const pageSize = ref(10)

// 获取分支列表
const branches = ref<Branch[]>([])
const curBranch = ref<Branch>()
const authors = ref<Author[]>([])
const commits = ref<Commit[]>([])
const init = async () => {
  let path = repo.value!.path
  const branchesPromise = getBranches(path)
  const curBranchPromise = getCurrentBranch(path)
  await Promise.allSettled([branchesPromise, curBranchPromise]).then(async (res) => {
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
  })
  getCommits()
}

const getCommits = _.debounce(async () => {
  let path = repo.value!.path
  loading.value = true
  // 这个函数需要有一个最小等待时间
  withMinDelay(async () => {
    const res = await getBranchCommits(path, curBranch.value!, 1 << 31)
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
})
// 监听路由变化，重新获取数据
watch(()=> route.path, () => {
  if (route.path.startsWith('/commit')) {
    id.value = parseInt(route.params.id as string)
    repo.value = repoStore.getRepoById(id.value)
    init().catch((err) => {
      window.$message.error(err.data) 
    }).finally(() => {
      // 重置分页,筛选刷新
      page.value = 1 
      filterModel.value = {
        author: null,
        content: null,
        timeRange: null
      }
    })
  }
}, {immediate: true})

// 筛选模型
const filterModel = ref<Model>({
  author: null,
  content: null,
  timeRange: null
})

// 提交记录总数
const total = computed(() => {
  return filtedList.value.length 
})

const filtedList = computed(() => {
  return commits.value.filter((commit) => {
    if (filterModel.value.author !== null) {
      return commit.authorName === filterModel.value.author 
    }
    return true
  }).filter((commit) => {
    return commit.message.includes(filterModel.value.content || '') 
  }).filter((commit) => {
    if (filterModel.value.timeRange === null) {
      return true 
    }
    return commit.datetime >= filterModel.value.timeRange[0] && commit.datetime <= filterModel.value.timeRange[1]
  })
})

const hasFilter = computed(() => {
  return filterModel.value.author !== null || filterModel.value.content !== null || filterModel.value.timeRange !== null 
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
      <FilterForm v-if="showFilter" :author-list="authors" v-bind:model-value="filterModel"></FilterForm>
    </template>
    <NFlex @contextmenu="handleContextMenu" :data-repo="repo?.path">
      <template v-for="c in filtedList.slice((page - 1) * pageSize, page * pageSize)">
        <CommitItem :commit="c"/>
      </template>
    </NFlex>
    <template #footer>
      <NFlex justify="center">
        <NPagination 
          :item-count="total" 
          v-model:page="page"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 30]"
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