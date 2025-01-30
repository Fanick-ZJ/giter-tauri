<script setup lang="ts">
import { useRepoStore, ValidRepository } from '@/store/modules/repo';
import RepoItem from './repo-item.vue'
import { NFlex, NDropdown } from 'naive-ui';
import { nextTick, ref } from 'vue';
import { upToRepoItem } from '../util';
import { useFileInfoDialog } from '@/components/info-dialog';
import { openFileManager } from '@/utils/tool';

defineOptions({
  name: 'RepoList'
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
  const repoEle = upToRepoItem(e.target as HTMLElement)
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
</script>

<template>
  <NFlex @contextmenu="handleContextMenu" vertical>
    <template v-for="item in repoStore.repos" :key="item.path">
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