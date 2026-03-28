import { Component, defineComponent, ref, h } from 'vue'
import { useAbstractDialog, DialogOptions, DialogCallbacks } from '../abstract-dialog'
import RepoSelector from './index.vue'
import { NModal } from 'naive-ui'

export interface RepoSelectorOptions {
  multiple?: boolean
}

export type RepoSelectorResult = string | string[]

export function useRepoSelectorDialog(props: RepoSelectorOptions) {
  let dialogActions: any;

  const options: DialogOptions = {
    containerName: '__repo__selector__container',
    buttonBox: 'custom',
    title: '',
    height: '650px',
    width: '900px'
  }

  const result = ref<RepoSelectorResult>()
  const show = ref(false)

  const content = (): Component => {
    return defineComponent({
      name: 'RepoSelectorDialog',
      setup() {
        const handleConfirm = (repos: RepoSelectorResult) => {
          result.value = repos
          show.value = false
          dialogActions?.setReturnData(repos)
          dialogActions?.ok()
        }

        const handleCancel = () => {
          show.value = false
          dialogActions?.close()
        }

        return () => (
          <div style="width: 100%; height: 100%;">
            <RepoSelector
              multiple={props.multiple}
              onConfirm={handleConfirm}
              onCancel={handleCancel}
            />
          </div>
        )
      }
    })
  }

  const callbacks: DialogCallbacks = {
    content
  }

  dialogActions = useAbstractDialog<RepoSelectorResult>(options, callbacks)

  return {
    ...dialogActions,
    showDialog: (): Promise<RepoSelectorResult> => {
      show.value = true
      return dialogActions.showDialog()
    }
  }
}

export function createRepoSelectorDialog(props: RepoSelectorOptions = {}): Promise<RepoSelectorResult> {
  const dialog = useRepoSelectorDialog(props)
  return dialog.showDialog()
}
