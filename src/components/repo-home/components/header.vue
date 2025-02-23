<script setup lang="ts">
import { useFileInfoDialog } from "@/components/common/info-dialog";
import { useRepoStore } from "@/store/modules/repo";
import { useNotificationStore } from "@/store/modules/notification";
import { Repository } from "@/types";
import { Icon } from "@iconify/vue";
import { NButton, NModal, NBadge, NFlex, NCard } from 'naive-ui'
import { computed, ref } from "vue";
import { createNofication } from "./notification";
import { isRepo } from "@/utils/command";
import { defaultRepository } from "@/types/util";
import { FilterModel } from "../types";
import RepoFilterForm from "./repo-filter-form.vue";
import { FileSelectorDialog } from "@/components/common/file-selector/index.tsx";

defineOptions({
  name: 'HomePageHeaders'
})
const repoStore = useRepoStore()
const notifStore = useNotificationStore()

const add = () => {
  const dlg = new FileSelectorDialog({directory: true})
  dlg.show()?.then(async (path) => {
    if (
        path === undefined 
      || path === '' 
      || (Array.isArray(path) && path.length === 0)) {
      window.$message.error('请选择仓库目录')
      return 
    }
    if (!Array.isArray(path)) {
      if (!await isRepo(path)) {
        window.$message.error(`请选择有效的仓库目录: ${path}`)
        return
      }
      useFileInfoDialog({path, mode: 'add'}).then((repo: Repository) => {
        repoStore.add(repo)
      })
    } else {
      for (const p of path) {
        if (!await isRepo(p)) {
          window.$message.error(`请选择有效的仓库目录: ${p}`)
          return
        }
      }
      for (const p of path) {
        repoStore.add(defaultRepository(p))
      }
    }
  })
}

const filterShow = ref(false)
const filterModel = ref<FilterModel>({
  alias: '',
  path: '',
  hasWatched: '',
  top: '',
  valid: ''
})
const showFilter = () => {
  filterShow.value = !filterShow.value
}

const handleFilter = (model: FilterModel) => {
  filterModel.value = model
  filterShow.value = false 
}

const notifShow = computed(() => {
  return notifStore.notifications.length > 0
})

const notifSize = computed(() => {
  return notifStore.notifications.length
})

const showMsg = () => {
  notifStore.notifications.forEach((notif) => {
    createNofication(notif)
  })
}

defineExpose({
 filter: filterModel 
})
</script>

<template>
  <div class="flex">
    <div class="flex-1 font-bold text-lg">
      仓库
    </div>
    <NFlex justify="end" class="w-[120px]" style="gap: 0">
      <NButton v-if="notifShow" quaternary circle @click="showMsg">
        <NBadge :value="notifSize" :max="99">
          <Icon icon="lets-icons:message-alt-duotone" width="24" height="24"  color="gray"/>
        </NBadge>
      </NButton>  
      <NButton quaternary circle @click="showFilter">
        <template #icon>
          <Icon icon="line-md:filter" width="30" height="30" color="gray"/>
        </template>
      </NButton>
      <NButton quaternary circle @click="add">
        <template #icon>
          <Icon icon="lets-icons:add-duotone" width="30" height="30" color="gray"/>
        </template>
      </NButton>
    </NFlex>
  </div>
  <NModal v-model:show="filterShow">
    <NCard title="仓库筛选" class="w-[80%]">
      <RepoFilterForm :model="filterModel" @filter="handleFilter"/>
    </NCard>
  </NModal>
</template>


<style scoped>

</style>