<script lang="ts" setup>
import Spliter from '@/components/common/spliter/index.vue'
import Editor from '@/components/common/editor/editor.vue'
import { useRoute } from 'vue-router';
import Tree from './components/tree.vue'
import { getBlobContent, objectIsBinary } from '@/utils/command';
import { ref } from 'vue';
import { bytesToString } from '@/utils/tool';

const route = useRoute()
const {repo: _repo, commitId: _commitId} = route.params;
let repo = _repo as string
let commitId = _commitId as string
console.log(repo, commitId)
const isBianry = ref(false)
const fileContent = ref("")
const selectedFile = async (object_id: string) => {
    if (await objectIsBinary(repo, object_id)) {
        isBianry.value = true
        console.error('暂不支持二进制文件')
    }
    let content = await getBlobContent(repo as string, object_id)
    fileContent.value = bytesToString(content)
    console.log(fileContent.value)

}
</script>

<template>
    <Spliter 
        class="h-screen" 
        :side_padding="10"
        :content_padding="10"
    >
        <template #sider>
            <Tree 
                :commit_id="commitId" 
                :repo="repo"
                @selected="selectedFile"
                />
        </template>
        <template #content>
            <Editor :content="fileContent" :readonly="true"/>
        </template>
    </Spliter>
</template>


<style lang="scss">
</style>