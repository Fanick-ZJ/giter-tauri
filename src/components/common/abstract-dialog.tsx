import { createSingletonComponent, InstanceManager } from "@/utils/tool";
import { NButton, NCard, NDialogProvider } from "naive-ui";
import { Component, defineComponent, h, nextTick, ref, Ref, computed } from "vue";

export type DialogOptions = {
  buttonBox?: 'cancel' | 'ok' | 'ok-cancel' | 'custom',
  containerName: string,
  title?: string | Ref<string>,
  subTitle?: string | Ref<string>,
  width?: string,
  height?: string,
  closeOnClickOutside?: boolean
}

export type DialogState<R> = {
  show: Ref<boolean>
  zIndex: Ref<number>
  returnData: Ref<R | undefined>
  resolve: Ref<((value: R | PromiseLike<R>) => void) | null>
  reject: Ref<((reason?: any) => void) | null>
  comp: Ref<InstanceManager<Component> | undefined>
}

export type DialogActions<R> = {
  showDialog: () => Promise<R>
  setReturnData: (data: R) => void
  setZIndex: (zIndex: number) => void
  ok: () => void
  close: () => void
  cleanup: () => Promise<void>
}

export type DialogCallbacks = {
  beforeOk?: () => boolean
  beforeClose?: () => void
  content: () => Component
  header?: () => Component
  customFooter?: () => Component
}

// 创建对话框 Hook
export function useAbstractDialog<R>(
  options: DialogOptions,
  callbacks: DialogCallbacks
): DialogActions<R> {
  // 状态管理
  const state: DialogState<R> = {
    show: ref(false),
    zIndex: ref(1000),
    returnData: ref<R | undefined>(undefined) as Ref<R | undefined>,
    resolve: ref<((value: R | PromiseLike<R>) => void) | null>(null),
    reject: ref<((reason?: any) => void) | null>(null),
    comp: ref<InstanceManager<Component> | undefined>(undefined)
  }

  // 清理方法
  const cleanup = async () => {
    state.show.value = false
    await nextTick()
    if (state.comp.value) {
      state.comp.value.unmount()
      state.comp.value = undefined
    }
  }

  // 操作方法
  const actions: DialogActions<R> = {
    showDialog: () => {
      try {
        state.comp.value = createSingletonComponent({
          className: options.containerName,
          component: createDialogComponent(options, state, actions, callbacks),
          props: {}
        })
      } catch (e) {
        window.$message.error('窗口实例已存在')
        return Promise.reject(new Error('窗口实例已存在'))
      }

      const promise = new Promise<R>((resolve, reject) => {
        state.resolve.value = resolve
        state.reject.value = reject
      })

      state.show.value = true
      return promise
    },

    setReturnData: (data: R) => {
      state.returnData.value = data
    },

    setZIndex: (zIndex: number) => {
      state.zIndex.value = zIndex
    },

    ok: () => {
      if (!callbacks.beforeOk?.()) {
        return
      }
      state.resolve.value?.(state.returnData.value!)
      cleanup()
    },

    close: () => {
      callbacks.beforeClose?.()
      state.reject.value?.(new Error('用户取消操作'))
      cleanup()
    },

    cleanup
  }

  return actions
}

// 创建对话框组件
function createDialogComponent<R>(
  options: DialogOptions,
  state: DialogState<R>,
  actions: DialogActions<R>,
  callbacks: DialogCallbacks
): Component {
  return defineComponent({
    name: 'AbstractDialog',
    setup() {
      // 头部组件
      const headerComponent = callbacks.header ? callbacks.header() : () => (
        <div class='flex w-max gap-2 overflow-x-hidden'>
          <div class='flex-1'>
            <div class='text-lg font-bold'>
              {options.title}
            </div>
          </div>
          <div class='text-sm flex items-end'>
            {options.subTitle}
          </div>
        </div>
      )

      // 底部组件
      const footerComponent = computed(() => {
        if (options.buttonBox === 'cancel') {
          return () => (
            <div class="flex justify-end">
              <NButton onClick={actions.close}>
                关闭
              </NButton>
            </div>
          )
        } else if (options.buttonBox === 'ok') {
          return () => (
            <div class="flex justify-end">
              <NButton onClick={actions.ok}>
                确定
              </NButton>
            </div>
          )
        } else if (options.buttonBox === 'ok-cancel') {
          return () => (
            <div class="flex justify-end gap-2">
              <NButton onClick={actions.ok} type='primary'>
                确定
              </NButton>
              <NButton onClick={actions.close}>
                取消
              </NButton>
            </div>
          )
        } else {
          return callbacks.customFooter?.()
        }
      })

      const slots = {
        default: () => h(callbacks.content()),
        header: () => h(headerComponent),
        action: () => footerComponent.value ? h(footerComponent.value) : undefined
      }

      const style = {
        width: options.width || '80%',
        height: options.height || '80%'
      }

      const closeOnClickOutside = options.closeOnClickOutside ?? true

      return () => (
        <>
          {state.show.value ? (
            <div
              style={{ zIndex: state.zIndex.value }}
              class='w-screen h-screen bg-slate-400/50 flex items-center justify-center fixed top-0 left-0'
              onClick={(e) => e.target === e.currentTarget && closeOnClickOutside && actions.close()}
            >
              <NDialogProvider>
                <NCard
                  style={style}
                  closable
                  onClose={actions.close}
                  v-slots={slots}
                  headerStyle={{ 'overflow-x': 'hidden', 'overflow-y': 'hidden' }}
                  contentStyle={{ 'overflow-y': 'hidden' }}
                >
                </NCard>
              </NDialogProvider>
            </div>
          ) : null}
        </>
      )
    }
  })
}

// 创建对话框的便捷函数
export function createDialog<R>(
  options: DialogOptions,
  callbacks: DialogCallbacks
): DialogActions<R> {
  return useAbstractDialog<R>(options, callbacks)
}