<script setup lang="ts">
import { useFileSelector } from "@/components/fileSelector";
import { Icon } from "@iconify/vue";
import { NButton, NIcon } from 'naive-ui'
defineOptions({
  name: 'HomePageHeaders'
})

const add = () => {
  useFileSelector({filter: (path: string) => {return true}}).then((res) => {
    console.log(res)
  })
}
</script>

<template>
  <div class="flex pb-[5px] items-center h-[30px]">
    <div class="flex-1 font-bold">
      仓库
    </div>
    <div class="w-[30px]">
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