<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, h } from 'vue'
import { Icon } from '@iconify/vue'
import {
  NCard,
  NButton,
  NInput,
  NBreadcrumb,
  NBreadcrumbItem,
  NGrid,
  NGi,
  NTooltip,
  NTag,
  NSpin,
  NEmpty,
  NScrollbar,
  NCheckbox,
  NAlert
} from 'naive-ui'
import { getDriver, getFolders, scanReposFolder, isRepo } from '@/utils/command'
import { T_Dir } from '../file-selector/types'

interface SelectedRepo {
  path: string
  name: string
  exists?: boolean
}

const props = defineProps<{
  multiple?: boolean
}>()

const emit = defineEmits<{
  confirm: [repos: string | string[]]
  cancel: []
}>()

// 状态管理
const loading = ref(false)
const currentPath = ref('')
const breadcrumbs = ref<Array<{ name: string; path: string }>>([])
const folders = ref<Array<{ name: string; path: string; is_repo: boolean }>>([])
const selectedRepos = ref<SelectedRepo[]>([])
const dragOverFolder = ref<string | null>(null)
const scanError = ref<string | null>(null)

// 已选仓库面板折叠状态
const selectedPanelCollapsed = ref(false)
const toggleSelectedPanel = () => {
  selectedPanelCollapsed.value = !selectedPanelCollapsed.value
}

// 获取驱动器列表
const loadDrives = async () => {
  loading.value = true
  try {
    const drives = await getDriver() as T_Dir[]
    folders.value = drives.map((drive: T_Dir) => ({
      name: drive.name,
      path: drive.path,
      is_repo: drive.is_repo
    }))
    breadcrumbs.value = [{ name: '此电脑', path: '' }]
    currentPath.value = ''
  } catch (error) {
    console.error('加载驱动器失败:', error)
  } finally {
    loading.value = false
  }
}

// 进入文件夹
const enterFolder = async (folder: { name: string; path: string; is_repo: boolean }) => {
  loading.value = true
  try {
    const result = await getFolders(folder.path) as T_Dir[]
    folders.value = result.map((dir: T_Dir) => ({
      name: dir.name,
      path: dir.path,
      is_repo: dir.is_repo
    }))
    breadcrumbs.value.push({ name: folder.name, path: folder.path })
    currentPath.value = folder.path
  } catch (error) {
    console.error('加载文件夹失败:', error)
  } finally {
    loading.value = false
  }
}

// 面包屑导航
const navigateToBreadcrumb = async (index: number) => {
  const target = breadcrumbs.value[index]
  if (index === 0) {
    await loadDrives()
  } else {
    await enterFolder({ name: target.name, path: target.path, is_repo: false })
  }
  breadcrumbs.value = breadcrumbs.value.slice(0, index + 1)
}

// 路径输入导航
const navigateToPath = async () => {
  if (!currentPath.value.trim()) {
    await loadDrives()
    return
  }

  loading.value = true
  try {
    const result = await getFolders(currentPath.value) as T_Dir[]
    folders.value = result.map((dir: T_Dir) => ({
      name: dir.name,
      path: dir.path,
      is_repo: dir.is_repo
    }))

    // 更新面包屑
    const parts = currentPath.value.split(/\/|\\/).filter(Boolean)
    breadcrumbs.value = [{ name: '此电脑', path: '' }]
    let buildPath = ''
    for (const part of parts) {
      buildPath += (buildPath ? '/' : '') + part
      breadcrumbs.value.push({ name: part, path: buildPath })
    }
  } catch (error) {
    window.$message.error('无法访问该路径')
  } finally {
    loading.value = false
  }
}

// 选择/取消选择仓库
const toggleRepoSelection = (folder: { name: string; path: string; is_repo: boolean }) => {
  if (!folder.is_repo) return

  const index = selectedRepos.value.findIndex(r => r.path === folder.path)
  if (index > -1) {
    selectedRepos.value.splice(index, 1)
  } else {
    selectedRepos.value.push({
      path: folder.path,
      name: folder.name
    })
  }
}

// 批量添加当前文件夹下的所有仓库
const addAllReposInFolder = async () => {
  if (!currentPath.value) {
    window.$message.warning('请先选择一个文件夹')
    return
  }

  scanError.value = null
  loading.value = true
  try {
    const repoPaths = await scanReposFolder(currentPath.value)

    if (repoPaths.length === 0) {
      window.$message.info('当前文件夹下没有找到 Git 仓库')
      return
    }

    // 添加到选中列表（去重）
    let addedCount = 0
    for (const path of repoPaths) {
      const name = path.split(/\/|\\/).pop() || path
      if (!selectedRepos.value.find(r => r.path === path)) {
        selectedRepos.value.push({ path, name })
        addedCount++
      }
    }

    if (addedCount > 0) {
      window.$message.success(`已添加 ${addedCount} 个仓库`)
    } else {
      window.$message.info('所有仓库都已存在于选择列表中')
    }
  } catch (error: any) {
    const errorMessage = error?.message || error?.toString() || '未知错误'
    scanError.value = `扫描失败: ${errorMessage}\n路径: ${currentPath.value}\n\n可能的原因:\n1. 路径不存在或无法访问\n2. 权限不足\n3. 路径格式错误`
    window.$message.error('扫描仓库失败，请查看错误详情')
  } finally {
    loading.value = false
  }
}

// 移除选中的仓库
const removeSelectedRepo = (path: string) => {
  const index = selectedRepos.value.findIndex(r => r.path === path)
  if (index > -1) {
    selectedRepos.value.splice(index, 1)
  }
}

// 确认添加
const confirmAdd = () => {
  if (selectedRepos.value.length === 0) {
    window.$message.warning('请至少选择一个仓库')
    return
  }

  const paths = selectedRepos.value.map(r => r.path)
  emit('confirm', props.multiple ? paths : paths[0])
  reset()
}

// 取消
const cancel = () => {
  emit('cancel')
  reset()
}

// 重置状态
const reset = () => {
  selectedRepos.value = []
  currentPath.value = ''
  breadcrumbs.value = []
}

// 拖拽相关
const handleDragOver = (folderPath: string) => {
  dragOverFolder.value = folderPath
}

const handleDragLeave = () => {
  dragOverFolder.value = null
}

const handleDrop = async (folder: { name: string; path: string; is_repo: boolean }) => {
  dragOverFolder.value = null

  if (folder.is_repo) {
    toggleRepoSelection(folder)
  } else {
    await enterFolder(folder)
  }
}

// 计算属性
const selectedCount = computed(() => selectedRepos.value.length)

// 检查屏幕宽度并自动设置折叠状态
const checkScreenSize = () => {
  const width = window.innerWidth
  // 在小屏幕下默认折叠已选面板
  if (width < 768) {
    selectedPanelCollapsed.value = true
  } else {
    selectedPanelCollapsed.value = false
  }
}

// 监听窗口大小变化
const handleResize = () => {
  checkScreenSize()
}

onMounted(() => {
  loadDrives()
  checkScreenSize()
  window.addEventListener('resize', handleResize)
})

// 清理事件监听
onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<template>
  <div class="repo-selector-container">
    <NCard
      title="添加仓库"
      :bordered="false"
      class="repo-selector-card"
      :segmented="{
        content: 'soft'
      }"
    >
      <!-- 面包屑导航 -->
      <template #header-extra>
        <NBreadcrumb class="cursor-pointer">
          <NBreadcrumbItem
            v-for="(item, index) in breadcrumbs"
            :key="index"
            @click="navigateToBreadcrumb(index)"
          >
            {{ item.name }}
          </NBreadcrumbItem>
        </NBreadcrumb>
      </template>

      <!-- 路径输入栏 -->
      <div class="flex gap-2 mb-4 flex-col sm:flex-row">
        <NInput
          v-model:value="currentPath"
          placeholder="输入路径或点击文件夹浏览"
          clearable
          @keyup.enter="navigateToPath"
          class="flex-1"
        >
          <template #prefix>
            <Icon icon="lucide:folder" width="16" height="16" />
          </template>
        </NInput>
        <NButton type="primary" @click="navigateToPath" class="sm:w-auto w-full">
          <template #icon>
            <Icon icon="lucide:arrow-right" width="16" height="16" />
          </template>
          转到
        </NButton>
      </div>

      <!-- 主内容区域 -->
      <div class="main-content-area flex gap-3 flex-1 min-h-0 flex-col lg:flex-row">
        <!-- 文件夹网格 -->
        <div class="flex-1 min-w-0 lg:min-w-0 flex flex-col min-h-0">
          <div class="flex-1 min-h-0">
            <NSpin :show="loading" style="height: 100%">
              <div v-if="folders.length === 0" class="h-full flex items-center justify-center">
                <NEmpty description="此文件夹为空" />
              </div>
              <div v-else class="h-full flex flex-col">
                <NScrollbar style="height: 100%">
              <NGrid
                :x-gap="12"
                :y-gap="12"
                :cols="2"
                :xs="2"
                :sm="3"
                :md="4"
                :lg="5"
                :xl="6"
                responsive="screen"
                class="folder-grid"
              >
                <NGi v-for="folder in folders" :key="folder.path">
                  <div
                    :class="[
                      'folder-item',
                      {
                        'is-repo': folder.is_repo,
                        'is-selected': selectedRepos.some(r => r.path === folder.path),
                        'is-dragover': dragOverFolder === folder.path
                      }
                    ]"
                    draggable="true"
                    @click="enterFolder(folder)"
                    @dragover.prevent="handleDragOver(folder.path)"
                    @dragleave="handleDragLeave"
                    @drop.prevent="handleDrop(folder)"
                  >
                    <div class="folder-checkbox" v-if="folder.is_repo" @click.stop>
                      <NCheckbox
                        :checked="selectedRepos.some(r => r.path === folder.path)"
                        @update:checked="() => toggleRepoSelection(folder)"
                      />
                    </div>
                    <div class="folder-icon">
                      <Icon
                        :icon="folder.is_repo ? 'lucide:folder-git-2' : 'lucide:folder'"
                        :width="40"
                        :height="40"
                        :color="folder.is_repo ? '#f59e0b' : '#94a3b8'"
                      />
                    </div>
                    <div class="folder-name">
                      {{ folder.name }}
                    </div>
                  </div>
                </NGi>
              </NGrid>
            </NScrollbar>
            </div>
          </NSpin>
          </div>
        </div>

        <!-- 已选仓库列表 -->
        <div
          class="selected-repos-panel w-full lg:w-64 border-t lg:border-t-0 lg:border-l flex flex-col min-h-0"
          :class="[
            selectedPanelCollapsed ? 'max-h-[50px]' : 'max-h-[40vh] lg:max-h-none'
          ]"
        >
          <div class="flex items-center justify-between mb-2">
            <div class="font-medium text-sm flex items-center gap-2">
              <span>已选仓库</span>
              <button
                @click="toggleSelectedPanel"
                class="lg:hidden text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors"
                title="切换面板显示"
              >
                <Icon :icon="selectedPanelCollapsed ? 'lucide:chevron-down' : 'lucide:chevron-up'" width="16" height="16" />
              </button>
            </div>
            <div class="flex items-center gap-2">
              <NTag size="small" :type="selectedCount > 0 ? 'info' : 'default'">
                {{ selectedCount }}
              </NTag>
              <button
                @click="toggleSelectedPanel"
                class="hidden lg:block text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors"
                title="切换面板显示"
              >
                <Icon :icon="selectedPanelCollapsed ? 'lucide:chevron-down' : 'lucide:chevron-up'" width="16" height="16" />
              </button>
            </div>
          </div>

          <div v-if="!selectedPanelCollapsed" class="mb-2">
            <NButton
              v-if="currentPath"
              size="small"
              type="primary"
              ghost
              block
              :loading="loading"
              @click="addAllReposInFolder"
            >
              <template #icon>
                <Icon icon="lucide:folder-plus" width="14" height="14" />
              </template>
              添加当前文件夹所有仓库
            </NButton>

            <!-- 路径提示 -->
            <div v-if="currentPath && !selectedPanelCollapsed" class="text-xs text-gray-400 mt-2 p-2 bg-gray-50 rounded">
              <div class="font-medium">当前路径:</div>
              <div class="truncate">{{ currentPath }}</div>
            </div>

            <NAlert
              v-if="scanError && !selectedPanelCollapsed"
              type="error"
              :title="scanError"
              closable
              @close="scanError = null"
              class="mt-3"
            />
          </div>

          <div v-if="!selectedPanelCollapsed" class="flex-1 min-h-0 flex flex-col">
            <NScrollbar style="height: 100%">
            <div v-if="selectedRepos.length === 0" class="text-center text-gray-400 py-8">
              <Icon icon="lucide:folder-open" width="48" height="48" class="mb-2" />
              <div class="text-sm">未选择任何仓库</div>
              <div class="text-xs mt-2 text-left px-4">
                <div class="font-medium mb-1">选择方式:</div>
                <ul class="space-y-1">
                  <li>• 单击文件夹进入查看</li>
                  <li>• 勾选 Git 仓库复选框</li>
                  <li>• 使用"添加当前文件夹所有仓库"按钮</li>
                </ul>
              </div>
            </div>
            <div v-else class="space-y-2">
              <div
                v-for="repo in selectedRepos"
                :key="repo.path"
                class="selected-repo-item"
              >
                <div class="flex items-center gap-2 flex-1 min-w-0">
                  <Icon icon="lucide:folder-git-2" width="16" height="16" color="#f05032" />
                  <div class="flex-1 min-w-0">
                    <div class="font-medium text-sm truncate">{{ repo.name }}</div>
                    <div class="text-xs text-gray-400 truncate">{{ repo.path }}</div>
                  </div>
                </div>
                <NButton
                  size="tiny"
                  quaternary
                  type="error"
                  @click="removeSelectedRepo(repo.path)"
                >
                  <template #icon>
                    <Icon icon="lucide:x" width="14" height="14" />
                  </template>
                </NButton>
              </div>
            </div>
          </NScrollbar>
          </div>
        </div>
      </div>

      <!-- 底部操作按钮 -->
      <template #footer>
        <div class="flex flex-col sm:flex-row justify-between items-center gap-3 sm:gap-0">
          <div class="text-sm text-gray-400 text-center sm:text-left">
            <template v-if="selectedCount > 0">
              已选择 {{ selectedCount }} 个仓库
            </template>
            <template v-else>
              单击进入文件夹，勾选选择仓库
            </template>
          </div>
          <div class="flex gap-2 w-full sm:w-auto">
            <NButton @click="cancel" class="flex-1 sm:flex-auto">取消</NButton>
            <NButton
              type="primary"
              :disabled="selectedCount === 0"
              @click="confirmAdd"
              class="flex-1 sm:flex-auto"
            >
              确定添加
            </NButton>
          </div>
        </div>
      </template>
    </NCard>
  </div>
</template>

<style scoped>
.repo-selector-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 280px;
  min-height: 400px;
}

.repo-selector-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  border: 1px solid #e2e8f0;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.repo-selector-card :deep(.n-card__content) {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.repo-selector-card :deep(.n-card__footer) {
  flex-shrink: 0;
  border-top: 1px solid #e2e8f0;
  background: #f8fafc;
  padding: 12px 16px;
}

/* 小屏幕优化 */
@media (max-width: 768px) {
  .repo-selector-card :deep(.n-card__footer) {
    padding: 8px 12px;
  }

  .repo-selector-card :deep(.n-card__footer) .flex {
    flex-direction: column;
    gap: 8px;
  }

  .repo-selector-card :deep(.n-card__footer) .flex.justify-between {
    flex-direction: row;
  }

  .repo-selector-card :deep(.n-card__footer) .flex.gap-2 {
    flex-direction: row;
    width: 100%;
  }

  .repo-selector-card :deep(.n-card__footer) button {
    flex: 1;
  }
}

:deep(.dark) .repo-selector-card .n-card__footer {
  border-top: 1px solid #334155;
  background: #1e293b;
}

.main-content-area {
  overflow: hidden;
}

.selected-repos-panel {
  overflow: hidden;
  transition: all 0.3s ease-in-out;
}

.folder-grid {
  padding: 8px;
}

/* 折叠按钮样式 */
.collapse-toggle {
  transition: transform 0.3s ease;
}

.collapse-toggle:hover {
  transform: scale(1.1);
}

/* 小屏幕优化：已选面板默认折叠状态 */
@media (max-width: 480px) {
  .selected-repos-panel {
    border-bottom: 1px solid #e2e8f0;
    background: #fafafa;
  }

  :deep(.dark) .selected-repos-panel {
    border-bottom-color: #334155;
    background: #1e293b;
  }
}

/* 折叠/展开动画 */
.selected-repos-panel {
  transition: max-height 0.3s ease-in-out, padding 0.3s ease-in-out;
}

/* 小屏幕下，默认给文件浏览区域更多空间 */
@media (max-width: 768px) {
  .flex-1.min-w-0:first-child {
    flex: 2 !important;
    min-height: 40vh !important;
  }

  .selected-repos-panel {
    flex: 0 0 auto !important;
  }
}

/* 确保NSpin内容区域能够正确计算高度 */
:deep(.n-spin-content) {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.folder-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 8px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid #e2e8f0;
  background: #ffffff;
  position: relative;
  user-select: none;
}

.folder-item:hover {
  background: #f8fafc;
  border-color: #cbd5e1;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.folder-item.is-repo {
  background: #fffbf7;
  border-color: #fed7aa;
}

.folder-item.is-selected {
  border-color: #3b82f6;
  background: #eff6ff;
}

.folder-item.is-dragover {
  border-color: #10b981;
  background: #f0fdf4;
}

.folder-checkbox {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 1;
}

.folder-icon {
  position: relative;
  margin-bottom: 8px;
}

.folder-name {
  font-size: 12px;
  text-align: center;
  word-break: break-all;
  line-height: 1.4;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.selected-repo-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 6px;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  transition: all 0.2s ease;
}

.selected-repo-item:hover {
  background: #f1f5f9;
  border-color: #cbd5e1;
}

/* 暗色主题适配 */
:deep(.dark) .folder-item {
  background: #1e293b;
  border-color: #334155;
}

:deep(.dark) .folder-item:hover {
  background: #334155;
  border-color: #475569;
}

:deep(.dark) .folder-item.is-repo {
  background: #2d1f15;
  border-color: #78350f;
}

:deep(.dark) .folder-item.is-selected {
  background: #1e3a8a;
  border-color: #3b82f6;
}

:deep(.dark) .selected-repo-item {
  background: #1e293b;
  border-color: #334155;
}

:deep(.dark) .selected-repo-item:hover {
  background: #334155;
  border-color: #475569;
}

/* 暗色主题边框优化 */
:deep(.dark) .repo-selector-card {
  border-color: #334155;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3);
}

@media (max-width: 768px) {
  .folder-grid {
    padding: 4px;
  }

  .folder-item {
    padding: 6px 4px;
  }

  .folder-icon {
    margin-bottom: 4px;
  }

  .folder-name {
    font-size: 11px;
  }

  .selected-repo-item {
    padding: 6px 8px;
    font-size: 12px;
  }

  /* 已选仓库面板优化 */
  .selected-repos-panel {
    position: relative;
    z-index: 10;
  }

  .selected-repos-panel.max-h-\[40vh\] {
    max-height: 35vh !important;
  }

  /* 路径输入区域响应式 */
  .flex.gap-2.mb-4 {
    gap: 8px !important;
  }

  /* 面包屑响应式 */
  .repo-selector-card :deep(.n-breadcrumb) {
    font-size: 12px;
  }

  /* 按钮文字响应式 */
  .repo-selector-card :deep(.n-button__content) {
    font-size: 12px;
  }

  /* 主内容区域优化：给文件浏览更多空间 */
  .main-content-area {
    flex-direction: column !important;
    gap: 8px !important;
  }

  .flex-1.min-w-0.lg\:min-w-0:first-child {
    flex: 2 !important;
    min-height: 50vh !important;
  }

  /* 已选面板在折叠时样式 */
  .selected-repos-panel[class*="max-h-[50px]"] {
    border-bottom: 1px solid #e2e8f0;
    background: #fafafa;
  }

  :deep(.dark) .selected-repos-panel[class*="max-h-[50px]"] {
    border-bottom-color: #334155;
    background: #1e293b;
  }

  /* 标题文字简化 */
  .repo-selector-card :deep(.n-card__header) {
    padding: 12px 16px;
  }

  .repo-selector-card :deep(.n-card__header .n-card__title) {
    font-size: 14px;
  }
}

@media (max-width: 480px) {
  .folder-item {
    padding: 4px 2px;
  }

  .folder-icon {
    margin-bottom: 2px;
  }

  .folder-checkbox {
    top: 2px;
    right: 2px;
  }

  .folder-name {
    font-size: 10px;
  }

  .selected-repos-panel {
    max-height: 30vh;
  }

  .repo-selector-card :deep(.n-card__footer) {
    padding: 6px 8px;
  }

  /* 极小屏幕隐藏部分内容 */
  .folder-checkbox {
    transform: scale(0.8);
  }

  .selected-repo-item {
    padding: 4px 6px;
  }

  /* 确保按钮在极小屏幕上也能正常显示 */
  .repo-selector-card :deep(.n-button) {
    min-width: 60px;
    font-size: 11px;
  }

  /* 小屏幕下自动折叠，给文件浏览更多空间 */
  .selected-repos-panel {
    max-height: 50px !important;
    overflow: hidden;
  }

  /* 进一步优化文件浏览区域 */
  .flex-1.min-w-0:first-child {
    min-height: 60vh !important;
    flex: 3 !important;
  }

  /* 面包屑文字大小 */
  .repo-selector-card :deep(.n-breadcrumb-item__link) {
    font-size: 11px;
    padding: 0 4px;
  }
}

@media (max-width: 360px) {
  /* 极小屏幕的额外优化 */
  .main-content-area {
    gap: 4px !important;
  }

  .folder-grid {
    gap: 4px !important;
    padding: 2px !important;
  }

  .repo-selector-container {
    min-width: 260px;
  }

  /* 文件浏览区域最大化 */
  .flex-1.min-w-0:first-child {
    min-height: 70vh !important;
  }

  /* 标题更紧凑 */
  .repo-selector-card :deep(.n-card__header) {
    padding: 8px 12px;
  }

  .repo-selector-card :deep(.n-card__footer) {
    padding: 4px 8px;
  }
}
</style>
