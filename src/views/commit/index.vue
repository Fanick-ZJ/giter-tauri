<script setup lang="ts">
import { useRepoStore } from '@/store/modules/repo';
import { computed, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { Icon } from '@iconify/vue'
import LayoutPage from '@/components/common/layout-page/index.vue'
import { getAuthors, getBranchCommits, getBranches, getCurrentBranch } from '@/utils/command';
import { Author, Branch, Commit, Repository } from '@/types';
import CommitItem from './components/commit-item.vue'
import { NFlex, NPagination, NButton, NIcon, NSelect, NDropdown } from 'naive-ui';
import { Model } from './type';
import FilterForm from './components/filter-form.vue'
import LoadingView from '@/components/common/loading-view.vue'
import { useContextMenu } from './hook';

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
  branches.value = await getBranches(path)
  curBranch.value = await getCurrentBranch(path)
  getCommits()
  authors.value = await getAuthors(repo.value!.path, curBranch.value)
}

const getCommits = async () => {
  let path = repo.value!.path
  loading.value = true
  getBranchCommits(path, curBranch.value!, 1 << 31).then((res) => {
    commits.value = res
    loading.value = false
  })
}

// 监听路由变化，重新获取数据
watch(()=> route.path, () => {
  if (route.path.startsWith('/commit')) {
    id.value = parseInt(route.params.id as string)
    repo.value = repoStore.getRepoById(id.value)
    init().catch((err) => {
      window.$message.error(err) 
    }).then(() => {
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
  <LayoutPage title="提交记录" :subtitle="repo?.alias">
    <template #header-extra>
      <div class="flex gap-x-[10px]">
        <NSelect v-model:value="selectedBranch" clearable :options="branchOptions" placeholder="选择分支" class="w-[200px]">
        </NSelect>
        <NButton :dashed='hasFilter' @click="toggleFilter">
          <NIcon>
            <Icon icon="mdi:filter-outline" width="100" height="100" />
          </NIcon>
        </NButton>
      </div>
    </template>
    <template #filter-form>
      <FilterForm v-if="showFilter" :author-list="authors" v-bind:model-value="filterModel"></FilterForm>
    </template>
    <LoadingView :loading="loading">
      <NFlex @contextmenu="handleContextMenu" :data-repo="repo?.path">
        <template v-for="c in filtedList.slice((page - 1) * pageSize, page * pageSize)">
          <CommitItem :commit="c"/>
        </template>
      </NFlex>
    </LoadingView>
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