<script setup lang="ts">
import { NSpace, TreeInst, NTree, TreeOption, TreeOverrideNodeClickBehavior, treeGetClickTarget, NDropdown, useDialog } from 'naive-ui';
import { TreeDir } from '@/types';
import { ComponentPublicInstance, computed, h, nextTick, onMounted, Ref, ref } from 'vue';
import { fileHistory, getTree } from '@/utils/command';
import { TreeFileMode } from '@/enum';
import { useElementSize } from '@vueuse/core';
import FileIcon from '@/components/common/file-icon/index.vue'
import FileHistoryWindow from '@/windows/file-history';
import { showFileHistory } from '@/utils/dialog';

const KEY_INTERVAL = '|KEY_INTERVAL|'
const props = defineProps<{
    commit_id: string,
    repo: string
}>()
const dialog = useDialog()

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

// 定义node-props, 可以在其中定义一些触发函数
const nodeProps = ({option}: {option: TreeOption}) => {
    return {
        // 记录右键点击时的option对象的filename和object_id
        onContextmenu: (e) => {
            if (option.isLeaf) {
                selectedRef.value = (option.key as string).split(KEY_INTERVAL)
            }
        }
    }
}

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

const useDropDown = () => {
    const selectedRef: Ref<string[]> = ref([])
    const showDropdownRef = ref(false)
    const xRef = ref(0)
    const yRef = ref(0)
    const menus = ref([
        {
            label: '文件历史',
            key: 'history'
        }
    ])
    const selectRefClear = () => {
        selectedRef.value = []
    }

    const handleSelected = (key: string) => {
        showDropdownRef.value = false
        let [path, object_id] = selectedRef.value
        showFileHistory(dialog, props.repo, path, props.commit_id)
        selectRefClear()

    }

    const handleContextMenu = (e: MouseEvent) => {
        if (selectedRef.value.length > 0) {
            showDropdownRef.value = true
            nextTick(() => {
                xRef.value = e.clientX
                yRef.value = e.clientY
            })
        }
    }

    const clickOutside = (e: MouseEvent) => {
        showDropdownRef.value = false
        selectRefClear()
    }
    return {
        selectedRef,
        showDropdownRef,
        xRef,
        yRef,
        menus,
        handleSelected,
        handleContextMenu,
        clickOutside
    }
}
const {selectedRef, showDropdownRef, xRef, yRef, menus, handleSelected, handleContextMenu, clickOutside} = useDropDown()
const emit = defineEmits(['selected'])
</script>

<template>
    <div class="h-full" ref="treeWrapperRef">
        <NSpace vertical>
            <NTree 
                ref="treeInstRef"
                :data="data"
                show-line
                block-line
                virtual-scroll
                @contextmenu="handleContextMenu"
                :style="treeStyle"
                :scrollbar-props="{
                    xScrollable: true
                }"
                :override-default-node-click-behavior="nodeClicked"
                :node-props="nodeProps"
            />
            <NDropdown
                placement="bottom-start"
                trigger="manual"
                :x="xRef"
                :y="yRef"
                @select="handleSelected"
                @clickoutside="clickOutside"
                :options="menus"
                :show="showDropdownRef">
            </NDropdown>
        </NSpace>
    </div>
</template>