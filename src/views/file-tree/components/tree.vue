<script setup lang="ts">
import { NSpace, TreeInst, NTree, TreeOption, TreeOverrideNodeClickBehavior } from 'naive-ui';
import { TreeDir } from '@/types';
import { ComponentPublicInstance, computed, h, onMounted, Ref, ref } from 'vue';
import { getTree } from '@/utils/command';
import { TreeFileMode } from '@/enum';
import { useElementSize } from '@vueuse/core';
import FileIcon from '@/components/common/file-icon/index.vue'

const KEY_INTERVAL = '|KEY_INTERVAL|'
const props = defineProps<{
    commit_id: string,
    repo: string
}>()

const treeInstRef = ref<ComponentPublicInstance<TreeInst> | null> (null)
const data: Ref<TreeOption[]> = ref([])
const treeWrapperRef = ref<HTMLElement>()
const createData = (tree: TreeDir): TreeOption[] => {
    const children: TreeOption[] = []
    tree.children.forEach(child => {
        let subchildren:TreeOption[] | undefined = []
        if (child.metadata.mode == TreeFileMode.Tree) {
            subchildren.push(...createData(child as TreeDir))
        } else {
            subchildren = undefined
        }
        let path = child.path == '' ? child.name: child.path + '/' + child.name
        children.push({
            label: child.name,
            key: path + KEY_INTERVAL + child.metadata.object_id,
            children: subchildren,
            isLeaf: child.metadata.mode != TreeFileMode.Tree,
            prefix: () =>  h(FileIcon, {pathOrName: child.name, width: 15, height: 15})
        })
    })
    return children
}

onMounted( async () => {
    const root_dir = await getTree(props.repo, props.commit_id)
    data.value = createData(root_dir)
})

const nodeClicked: TreeOverrideNodeClickBehavior = ({option})  => {
    let [path, object_id] = (option.key as string).split(KEY_INTERVAL)
    if (option.isLeaf) {
        emit('selected', path, object_id)
        return 'default'
    }
    else if (option.children!.length > 0) return 'toggleExpand'
    else {
        getTree(props.repo, object_id, path).then( tree => {
            option.children = createData(tree)
        })
        return 'toggleExpand'
    }
}
const useTreeStyle = () => {
    const {height} = useElementSize(treeWrapperRef)
    const treeStyle = computed(() => {
        return {
            height: `${height.value}px`
        }
    })
    return {
        treeStyle
    }
}
const { treeStyle } = useTreeStyle()
const emit = defineEmits(['selected'])
</script>

<template>
    <div class="h-full" ref="treeWrapperRef">
        <NSpace vertical>
            <NTree 
                ref="treeInstRef"
                :data="data"
                virtual-scroll
                :style="treeStyle"
                :scrollbar-props="{
                    xScrollable: true
                }"
                :override-default-node-click-behavior="nodeClicked"
            />
        </NSpace>
    </div>
</template>