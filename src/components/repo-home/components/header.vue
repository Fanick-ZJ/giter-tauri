<script setup lang="ts">
import { useFileInfoDialog } from "@/components/common/info-dialog";
import { useRepoStore } from "@/store/modules/repo";
import { useNotificationStore } from "@/store/modules/notification";
import { Repository } from "@/types";
import { Icon } from "@iconify/vue";
import { NButton, NModal, NBadge, NFlex, NCard, NTooltip } from 'naive-ui'
import { computed, onUnmounted, ref, watch } from "vue";
import { createNofication } from "./notification";
import { isRepo, getRepoByPath } from "@/utils/command";
import { defaultRepository } from "@/types/util";
import { FilterModel } from "../types";
import RepoFilterForm from "./repo-filter-form.vue";
import { createRepoSelectorDialog } from "@/components/common/repo-selector/index";

defineOptions({
  name: 'HomePageHeaders'
})
const repoStore = useRepoStore()
const notifStore = useNotificationStore()

const add = async () => {
  try {
    const result = await createRepoSelectorDialog({ multiple: true })

    if (!result) {
      window.$message.error('请选择仓库目录')
      return
    }

    console.log('添加仓库结果:', result)

    const paths = Array.isArray(result) ? result : [result]
    let addedCount = 0
    let skippedCount = 0

    for (const path of paths) {
      console.log('处理路径:', path)
      try {
        const existingRepo = await getRepoByPath(path)
        console.log('已存在仓库:', existingRepo)
        if (existingRepo === undefined || existingRepo === null) {
          const newRepo = defaultRepository(path)
          console.log('添加新仓库:', newRepo)
          repoStore.add(newRepo)
          addedCount++
          // 等待一小段时间让异步操作完成
          await new Promise(resolve => setTimeout(resolve, 500))
        } else {
          skippedCount++
        }
      } catch (err) {
        console.error('处理仓库时出错:', path, err)
        window.$message.error(`添加仓库 ${path} 失败: ${err instanceof Error ? err.message : '未知错误'}`)
      }
    }

    if (addedCount > 0) {
      window.$message.success(`成功添加 ${addedCount} 个仓库${skippedCount > 0 ? `，跳过 ${skippedCount} 个已存在的仓库` : ''}`)
    } else if (skippedCount > 0) {
      window.$message.info(`所有仓库已存在`)
    }
  } catch (error) {
    // 用户取消操作，不显示错误
    console.log('取消添加仓库或发生错误:', error)
  }
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

const sortStatusClick = ref(false)
const sortIcon = computed(() => {
  return sortStatusClick.value? 'eos-icons:content-modified' : 'garden:sort-fill-12'
})
const sortByStatus = () => {
  sortStatusClick.value = !sortStatusClick.value
  if (sortStatusClick.value) {
    repoStore.sortByStatus()
  } else {
    repoStore.defaultSort()
  }
}

const cancelStatusChangedCb = repoStore.addStatusChangeCb((path, status) => {
  if (sortStatusClick.value) {
    repoStore.sortByStatus()
  }
})

onUnmounted(() => {
  cancelStatusChangedCb()
})



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
      <NTooltip>
        <template #trigger>
          <NButton quaternary circle @click="sortByStatus">
            <template #icon>
              <Icon :icon="sortIcon" width="24" height="24" />
            </template>
          </NButton>
        </template>
        按状态排序
      </NTooltip>
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