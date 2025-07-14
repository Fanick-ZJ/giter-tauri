<script lang="ts" setup>
import { LayoutInst, LayoutSiderInst, NLayout, NLayoutSider } from 'naive-ui'
import _ from 'lodash'
import { ComponentPublicInstance, computed, getCurrentInstance, onMounted, ref } from 'vue'
const props = defineProps({
    direction: {
        type: String,
        required: false,
        default: 'vertical'
    },
    side_padding: {
        type: Number,
        required: false,
        default: 0
    },
    side_min_width: {
        type: Number,
        required: false,
        default: 100
    },
    side_min_height: {
        type: Number,
        required: false,
        default: 100
    },
    side_max_width: {
        type: Number,
        required: false,
        default: 400
    },
    side_max_height: {
        type: Number,
        required: false,
        default: 400
    },
    side_width: {
        type: Number,
        required: false,
        default: 200
    },
    side_height: {
        type: Number,
        required: false,
        default: 200
    },
    side_bar_size: {
        type: Number,
        required: false,
        default: 3
    },
    content_padding: {
        type: Number,
        required: false,
        default: 0
    },
    sidable: {
        type: Boolean,
        required: false,
        default: true
    }
})

const isVertical = props.direction == 'vertical'
// 1. 获取当前组件实例
const instance = getCurrentInstance()

// 2. 检查父组件是否传入了 side_width
const hasSideSizeProp = computed(() => {
    let field = 'side-width'
    if (!isVertical) {
        field = 'horizontal'
    }
    return instance?.vnode.props?.[field] !== undefined
  // 注意：HTML 模板中 prop 名是 kebab-case（side-width），不是 camelCase（sideWidth）
})

const useSizeProps = () => {
    const hasMaxSizeProp = computed(() => {
        let field = 'side_max_width'
        if (!isVertical) {
            field = 'side_max_height'
        }
        return instance?.vnode.props?.[field] !== undefined
    })

    const hasMinSizeProp = computed(() => {
        let field = 'side_min_width'
        if (!isVertical) {
            field = 'side_min_height'
        }
        return instance?.vnode.props?.[field] !== undefined
    })

    const maxSize = computed(() => {
        if (!isVertical) {
            return hasMaxSizeProp.value ? props.side_max_height : window.innerHeight - 50
        } else {
            return hasMaxSizeProp.value ? props.side_max_width : window.innerWidth - 50
        }
    })

    const minSize = computed(() => {
        if (!isVertical) {
            return hasMinSizeProp.value ? props.side_min_height : 50
        } else {
            return hasMinSizeProp.value ? props.side_min_width : 50
        }
    })

    return {
        minSize,
        maxSize
    }
}

const { minSize, maxSize} = useSizeProps()

// 3. 本地状态（初始为 props.side_width）
const localSize = ref<number>(isVertical ? props.side_width : props.side_height)

// 4. 实际使用的宽度（优先用本地状态，除非父组件传入了值）
const actualSize = computed(() => {
    let size = isVertical ? props.side_width : props.side_height
    return hasSideSizeProp.value ? size : localSize.value
})



const useStyle = () => {
    let content_style = computed(() => {
        return {
            padding: `${props.content_padding}px`,
        }
    })
    let sider_style = computed(() => {
        if (isVertical) {
            return {
                padding: `${props.side_padding}px`,
                width: `${actualSize.value}px`,
                minWidth: `${props.side_min_width}px`
            }
        } else {
            return {
                padding: `${props.side_padding}px`,
                height: `${actualSize.value}px`,
                minWidth: `${props.side_min_width}px`
            }
        }
    })
    return {
        content_style,
        sider_style
    }
}

const siderRef = ref<HTMLElement | null>(null)
const contentRef = ref<HTMLElement | null>(null)
const spliterBarRef = ref<HTMLElement | null>(null)

const useSidebar = () => {
    const sider_bar = spliterBarRef.value!
    if (isVertical) {
        sider_bar.style.cursor = 'col-resize'
        sider_bar.style.width = `${props.side_bar_size}px`
    } else {
        sider_bar.style.cursor = 'row-resize'
        sider_bar.style.height = `${props.side_bar_size}px`
    }

    let is_dragging = false
    let startMousePos = 0
    let startSize = 0

    const handleMouseUp = () => {
        is_dragging = false
        sider_bar.style.backgroundColor = ''
        document.removeEventListener('mousemove', handleMouseMove)
        document.removeEventListener('mouseup', handleMouseUp)
    }

    const handleMouseMove = (event: MouseEvent) => {
        if (!is_dragging) return
        const currentMousePos = isVertical ? event.clientX : event.clientY
        const delta = currentMousePos - startMousePos
        const newSize = startSize + delta
        const clampedSize = Math.max(minSize.value, Math.min(maxSize.value, newSize))

        if (hasSideSizeProp.value) {
            emit('update:side_size', clampedSize)
        } else {
            localSize.value = clampedSize
        }
    }

    const handleMouseDown = (event: MouseEvent) => {
        is_dragging = true
        sider_bar.style.backgroundColor = '#3b82f6'
        startMousePos = isVertical ? event.clientX : event.clientY
        startSize = actualSize.value
        document.addEventListener('mousemove', handleMouseMove)
        document.addEventListener('mouseup', handleMouseUp)
    }

    sider_bar.addEventListener('mousedown', handleMouseDown)
}

const {content_style, sider_style} = useStyle()
onMounted(() => {
    useSidebar()
})

const emit = defineEmits(['update:side_size'])
</script>

<template>
    <div class="flex" :class="isVertical ? '': 'flex-col'">
        <div ref="siderRef" :style="sider_style" class="shrink-0">
            <slot name="sider"></slot>
        </div>
        <div class="spliter__bar bg-gray-100" ref="spliterBarRef"></div>
        <div :style="content_style" ref="contentRef" class="w-full">
            <slot name="content"></slot>
        </div>
    </div>
</template>


<style lang="scss" scoped>
</style>