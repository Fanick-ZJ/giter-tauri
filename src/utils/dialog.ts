import { useDialog } from "naive-ui"
import { fileHistory } from "./command"
import { withMinDelay } from "./tool"
import FileHistoryWindow from "@/windows/file-history"
import { DialogApiInjection } from "naive-ui/es/dialog/src/DialogProvider"

// 对话框配置类型定义
type DialogConfig = {
    LOADING_DELAY: number
    TITLE: string
    POSITIVE_TEXT: string
    ERROR_TITLE: string
    ERROR_CONTENT: string
    ERROR_POSITIVE_TEXT: string
}

// 通用对话框选项类型
type DialogOptions = {
    title: string
    content: string
    positiveText: string
    maskClosable?: boolean
    onEsc?: () => void
    onClose?: () => void
    onPositiveClick?: () => void
}

// 对话框配置常量
const DIALOG_CONFIGS = {
    // 文件历史记录对话框配置
    FILE_HISTORY: {
        LOADING_DELAY: 500,
        TITLE: '获取历史记录中....',
        POSITIVE_TEXT: '取消',
        ERROR_TITLE: '错误',
        ERROR_CONTENT: '获取文件历史记录失败，请重试',
        ERROR_POSITIVE_TEXT: '确定'
    },
    // 可以在这里添加其他对话框配置
    // EXAMPLE_DIALOG: {
    //     TITLE: '示例对话框',
    //     POSITIVE_TEXT: '确定'
    // }
} as const

/**
 * 创建通用的加载对话框
 * @param dialog - Naive UI 对话框实例
 * @param options - 对话框选项
 * @returns 对话框实例和取消状态控制器
 */
const createLoadingDialog = (dialog: DialogApiInjection, options: DialogOptions) => {
    let cancelled = false
    
    const handleCancel = () => {
        cancelled = true
    }
    
    const dialogInstance = dialog.success({
        ...options,
        onEsc: handleCancel,
        onClose: handleCancel,
        onPositiveClick: handleCancel,
    })
    
    return {
        dialogInstance,
        isCancelled: () => cancelled,
        cancel: handleCancel
    }
}

/**
 * 显示文件历史记录对话框
 * @param dialog - Naive UI 对话框实例
 * @param repo - 仓库路径
 * @param file_abs_path - 文件绝对路径
 * @param commitId - 可选的提交ID
 * @returns 对话框实例
 */
export const showFileHistory = (
    dialog: DialogApiInjection, 
    repo: string, 
    file_abs_path: string, 
    commitId?: string
) => {
    const config = DIALOG_CONFIGS.FILE_HISTORY
    const historyHandle = withMinDelay(() => fileHistory(repo, file_abs_path), config.LOADING_DELAY)
    
    // 创建加载对话框
    const { dialogInstance, isCancelled } = createLoadingDialog(dialog, {
        title: config.TITLE,
        content: `正在获取 ${file_abs_path} 的历史记录`,
        positiveText: config.POSITIVE_TEXT,
        maskClosable: false,
    })
    
    // 处理异步结果
    historyHandle
        .then(res => {
            if (isCancelled()) return
            dialogInstance.destroy()
            FileHistoryWindow.addHistoryTab(repo, res, commitId)
        })
        .catch(error => {
            if (isCancelled()) return
            dialogInstance.destroy()
            // 错误处理
            console.error('获取文件历史记录失败:', error)
            dialog.error({
                title: config.ERROR_TITLE,
                content: config.ERROR_CONTENT,
                positiveText: config.ERROR_POSITIVE_TEXT
            })
        })
    
    return dialogInstance
}