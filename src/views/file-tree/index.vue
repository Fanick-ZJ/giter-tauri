<script lang="ts" setup>
import { commitTree } from '@/utils/command';
import { useRouter, useRoute } from 'vue-router';
const router = useRouter()
const route = useRoute()
const {repo, commitId} = route.params;
console.log(repo, commitId)
// console.log(await commitTree(repo, commitId))

</script>

<template>
<h1>HELLO THIS IS FILE TREE</h1>
</template>


<script lang="scss">
</script>