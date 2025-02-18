import { upToDataElement } from '@/utils/dom';
import { nextTick, ref } from "vue"
import { useCommitDetailDialog } from "./components/detail-dialog"

export const useContextMenu = () => {
  const menuX = ref(0)
  const menuY = ref(0)
  const showMenu = ref(false)
  const target = ref<HTMLElement>()
  const repo = ref('')
  const options = [
    {
      key: 'detail',
      label: '详情'
    }
  ]
  const handleContextMenu = (e: MouseEvent) => {
    e.preventDefault()
    showMenu.value = false
    nextTick(() => {
      const el = upToDataElement(e.target as HTMLElement, 'data-commit-id')
      const reopEl = upToDataElement(e.target as HTMLElement, 'data-repo')
      if (!el || !reopEl) {
        return 
      }
      target.value = el
      menuX.value = e.clientX
      menuY.value = e.clientY
      showMenu.value = true
      repo.value = reopEl.attributes.getNamedItem('data-repo')?.value || ''
    })
  }
  const menuCloseOutside = () => {
    showMenu.value = false
  }
  const handleSelect = (key: string) => {
    if (key === 'detail' && repo.value) {
      const commitId = target.value?.attributes.getNamedItem('data-commit-id')?.value || ''
      useCommitDetailDialog({
        commitId,
        repo: repo.value
      })
    }
    showMenu.value = false
  }
  return {
    menuX,
    menuY,
    showMenu,
    handleContextMenu,
    menuCloseOutside,
    handleSelect,
    options
  }
}