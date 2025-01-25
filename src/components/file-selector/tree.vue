<script setup lang="ts">
import { NTree, TreeOption, NSpace, TreeInst } from 'naive-ui';
import { h, onMounted, PropType, ref, useTemplateRef } from 'vue';
import { Folder, SelectFilter, T_Dir } from './types';
import { invoke } from '@tauri-apps/api/core';
import { Icon } from '@iconify/vue/dist/iconify.js';
import { SEPERATOR } from '@/const';
import { GET_DRIVER, GET_FOLDERS } from '@/const/command';

defineOptions({
  name: 'FileTree'
})

const props = defineProps({
  path: {
    type: String,
    required: false,
  },
  directory: {
    type: Boolean,
    required: false,
  },
  filter: {
    type: Object as PropType<SelectFilter>,
    required: false,
  },
  repoTip: {
    type: Boolean,
    required: false,
    default: true
  },
  root: {
    type: String,
    required: false,
  }
})

const options =ref<Folder[]>()
const defaultKeys = ref<string[]>([])
const treeRef = useTemplateRef<TreeInst>('treeRef')
let selected: string = ''

const getDrive = () => {
  invoke(GET_DRIVER).then((res) => {
    options.value = (res as [T_Dir]).map((dir) => {
      return {
        path: dir.path,
        name: dir.name,
        is_repo: false,
        isLeaf:false,
        children: undefined,
      }
    })
  })
}

const getDefault = async () => {
  let defaultKeys: string[] = []
  if (props.path) {
    const paths = props.path.split(SEPERATOR)
    let current: Folder
    // 获取第一个文件夹
    let index = options.value?.findIndex((dir) => {
      return dir.name === paths[0]
    })
    if (index !== -1) {
      current = options.value![index!]
      defaultKeys.push(current.path)
      // 逐级获取子文件夹
      for (let i = 1; i < paths.length; i++) {
        await handleLoad(current)
        let index = current.children?.findIndex((dir) => {
          return dir.name === paths[i]
        })
        if (index!== -1) {
          current = current.children![index!]
          defaultKeys.push(current.path)
        } else {
          break
        }
      }
    }
  }
  return defaultKeys
}

const scrollToDefault = () => {
  if (treeRef.value && defaultKeys.value.length > 0) {
    treeRef.value.scrollTo({
      key: defaultKeys.value[defaultKeys.value.length - 1],
    })
    treeRef.value.scrollTo({
      key: defaultKeys.value[defaultKeys.value.length - 1],
    })
  }
}

onMounted(async () => {
  getDrive()
  // 如果有path，逐级获取子文件夹
  defaultKeys.value = await getDefault()
  setTimeout(() => {
    scrollToDefault()
  }, 50)
})

// 不断的获取子文件夹
const handleLoad = async (option: TreeOption | Folder) => {
  // 官方文档写的不是很好，如果children不赋值的话，会一直重新调用这个函数
  option.children = [];
  return new Promise((resolve) => {
    invoke(GET_FOLDERS, {
      path: option.path,
    }).then((res) => {
      (res as Folder[]).forEach((dir) => {
        if (props.filter && !props.filter(dir.path)) return
        option.children?.push({
            path: dir.path,
            name: dir.name,
            is_repo: dir.is_repo,
            // 添加图标
            prefix: () => {
              const icon = props.directory
                          ? dir.is_repo && props.repoTip
                            ? "lucide:folder-git-2" 
                            : "cuida:folder-outline"
                          : "lucide:file";
              return h(Icon,{
                  icon: icon,
                  width: "20",
                  height: "20",
                  color: "gray"
                }
              )
            },
            children: undefined,
            isLeaf:false
          })
      })
    }).catch((err) => {
      console.log(err)
    }).finally(() => {
      resolve(true)
    })
  })
}

const handleSelect = ({ option }: {option: TreeOption}) => {
  const path = option.path as string
  selected = path
  emit('change', selected)
}

const emit = defineEmits<{
  change: [selected: string]
}>()

</script>

<template>
  <NSpace vertical :wrap="false">
    <!-- @vue-expect-error -->
    <NTree
      ref="treeRef"
      block-line
      :data="options"
      :key-field="'path'"
      :label-field="'name'"
      :block-node="true"
      :default-expanded-keys="defaultKeys"
      :override-default-node-click-behavior="handleSelect"
      style="height: 320px"
      virtual-scroll
      expand-on-click
      :on-load="handleLoad" />
  </NSpace>
</template>