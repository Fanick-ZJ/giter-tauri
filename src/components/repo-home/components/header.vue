<script setup lang="ts">
import { useFileSelector } from "@/components/file-selector";
import { useFileInfoDialog } from "@/components/info-dialog";
import { useRepoStore } from "@/store/modules/repo";
import { Repository } from "@/types/store";
import { Icon } from "@iconify/vue";
import { NButton } from 'naive-ui'
defineOptions({
  name: 'HomePageHeaders'
})
const store = useRepoStore()
const add = () => {
  useFileSelector({directory: true}).then((path) => {
    if (path === undefined) return
    useFileInfoDialog({path, mode: 'add'}).then((repo: Repository) => {
      store.add(repo)
    })
  })
}
</script>

<template>
  <div class="flex pb-[5px] items-center">
    <div class="flex-1 font-bold ] text-lg">
      仓库
    </div>
    <div class="w-[30px">
      <NButton quaternary circle @click="add">
      <template #icon>
        <Icon icon="lets-icons:add-duotone" width="30" height="30" color="gray"/>
      </template>
    </NButton>
    </div>
  </div>
</template>


<style scoped>

</style>