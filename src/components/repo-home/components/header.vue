<script setup lang="ts">
import { useFileSelector } from "@/components/file-selector";
import { useFileInfoDialog } from "@/components/info-dialog";
import { useRepoStore } from "@/store/modules/repo";
import { useNotificationStore } from "@/store/modules/notification";
import { Repository } from "@/types";
import { Icon } from "@iconify/vue";
import { NButton, NDropdown, NBadge, NFlex } from 'naive-ui'
import { computed } from "vue";
import { createNofication } from "./notification";
import { isRepo } from "@/utils/command";
defineOptions({
  name: 'HomePageHeaders'
})
const repoStore = useRepoStore()
const notifStore = useNotificationStore()

const add = () => {
  useFileSelector({directory: true}).then(async (path) => {
    if (path === undefined) return
    if (! await isRepo(path)) {
      window.$message.error('请选择仓库目录')
      return
    }
    useFileInfoDialog({path, mode: 'add'}).then((repo: Repository) => {
      repoStore.add(repo)
    })
  })
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
</script>

<template>
  <div class="flex pb-[5px] items-center">
    <div class="flex-1 font-bold text-lg">
      仓库
    </div>
    <NFlex justify="end" class="w-[80px]">
      <NButton quaternary circle @click="add">
        <template #icon>
          <Icon icon="lets-icons:add-duotone" width="30" height="30" color="gray"/>
        </template>
      </NButton>
      <NDropdown v-if="notifShow" trigger="hover">
        <NButton quaternary circle @click="showMsg">
          <NBadge :value="notifSize" :max="99">
            <Icon icon="lets-icons:message-alt-duotone" width="24" height="24"  color="gray"/>
          </NBadge>
        </NButton>
      </NDropdown>
    </NFlex>
  </div>
</template>


<style scoped>

</style>