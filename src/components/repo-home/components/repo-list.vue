<script setup lang="ts">
import { useRepoStore, ValidRepository } from '@/store/modules/repo';
import RepoItem from './repo-item.vue'
import { NFlex, NDropdown } from 'naive-ui';
import { computed, nextTick, PropType, ref, watch } from 'vue';
import { upToDataElement } from '@/utils/dom';
import { useFileInfoDialog } from '@/components/common/info-dialog';
import { openFileManager } from '@/utils/tool';
import { FilterModel } from '../types';

defineOptions({
  name: 'RepoList'
})

const props = defineProps({
 filter: {
  type: Object as PropType<FilterModel>,
  required: false
 } 
})
const repoStore = useRepoStore()

const x = ref<number>()
const y = ref<number>()
const options = [
  {
    label: '打开',
    key: 'open'
  },
  {
    label: '删除',
    key: 'delete'
  },
  {
    label: '更新',
    key: 'update' 
  }
]

const contextSelectItem = ref<ValidRepository | undefined>()

const showContext = ref(false)
const handleContextMenu = (e: MouseEvent) => {
  e.preventDefault()
  const repoEle = upToDataElement(e.target as HTMLElement, 'data-repo')
  if (!repoEle) return
  const path = repoEle!.getAttribute('data-repo')
  contextSelectItem.value = repoStore.getRepoByPath(path!)
  x.value = e.clientX
  y.value = e.clientY 
  showContext.value = false
  nextTick(() => {
    showContext.value = true
  })
}

const onClickoutside = () => {
  showContext.value = false 
}

const handleSelect = (key: string) => {
  showContext.value = false
  if (!contextSelectItem.value) return
  switch (key) {
    case 'update':
      const path = contextSelectItem.value.path
      const id = contextSelectItem.value.id
      useFileInfoDialog({path, id, mode: 'edit'}).then((res) => {
        repoStore.update(res)
      })
      break;
    case 'delete':
      let repo = repoStore.getRepoByPath(contextSelectItem.value.path)
      repo && repoStore.remove(repo)
      break;
    case 'open':
      openFileManager(contextSelectItem.value.path)
      break;
  }
}

const filtedRepos = ref<ValidRepository[]>(repoStore.repos)
watch(() => props.filter, (filter) => {
  if (!filter || Object.keys(filter).length == 0) return repoStore.repos
  let repos = repoStore.repos
  repos = repos.filter((repo) => {
    if (filter.alias && repo.alias.indexOf(filter.alias!) == -1) {
      return false
    }
    if (filter.path && repo.path.indexOf(filter.path!) == -1) {
      return false 
    }
    if (typeof filter.top != 'undefined') {
      if (filter.top == 'yes' && !repo.top) return false
      if (filter.top == 'no' && repo.top) return false
    }
    if (typeof filter.hasWatched != 'undefined') {
      if (filter.hasWatched == 'yes' &&!repo.hasWatch) return false
      if (filter.hasWatched == 'no' && repo.hasWatch) return false
    }
    if (typeof filter.valid != 'undefined') {
      if (filter.valid == 'yes' &&!repo.valid) return false
      if (filter.valid == 'no' && repo.valid) return false
    }
    if (filter.order && repo.order!= filter.order) {
      return false 
    }
    return true
  })
  filtedRepos.value = repos
}, {deep: true})
</script>

<template>
  <NFlex @contextmenu="handleContextMenu" vertical>
    <template v-for="item in filtedRepos" :key="item.path">
      <RepoItem :repo="item" />
    </template>
    <NDropdown
      placement="bottom-start"
      trigger="manual"
      :x="x"
      :y="y"
      :options="options"
      :on-clickoutside="onClickoutside"
      @select="handleSelect"
      :show="showContext"><div></div></NDropdown>
  </NFlex>
</template>



<style scoped>

</style>