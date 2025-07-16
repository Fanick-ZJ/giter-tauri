<script lang="ts" setup>
import Spliter from '@/components/common/spliter/index.vue'
import Editor from '@/components/common/editor/editor.vue'
import { useRoute } from 'vue-router';
import Tree from './components/tree.vue'
import { getBlobContent, objectIsBinary } from '@/utils/command';
import { ref } from 'vue';
import { basename, bytesToString } from '@/utils/tool';
import { now } from 'lodash-es';

const route = useRoute()
const {repo: _repo, commitId: _commitId} = route.params;
let repo = _repo as string
let commitId = _commitId as string

const isBianry = ref(false)
const fileContent = ref("")
const current_path = ref('')
const selectedFile = async (path: string, object_id: string) => {
    if (await objectIsBinary(repo, object_id)) {
        isBianry.value = true
        console.error('暂不支持二进制文件')
    }
    current_path.value = path
    let content = await getBlobContent(repo as string, object_id)
    fileContent.value = bytesToString(content)

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
            <Editor :filename="basename(current_path)" :content="fileContent" :readonly="true"/>
        </template>
    </Spliter>
</template>


<style lang="scss">
</style>