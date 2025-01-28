<script setup lang="ts">
import { NLayout, useMessage, useNotification } from 'naive-ui';
import RepoHome from './repo-home/index.vue'

defineOptions({
  name: 'HomePage'
})

// 在window上挂在一个message对象实例，方便使用
window.$message = useMessage()
window.$notification = useNotification()

</script>
<template>
  <NLayout>
    <RepoHome/>
  </NLayout>
</template>


<style scoped>

</style>