// 使用组件，通过函数调用的方式，将组件附着在root上
import {Component, computed, ComputedRef, defineComponent, nextTick, onBeforeMount, onBeforeUnmount, ref, watch } from 'vue'
import { SourceConterolDialogProps } from './types'
import { useAbstractDialog, DialogOptions, DialogCallbacks, createDialog } from '../abstract-dialog'
import { NButton, NDivider, NDropdown, NInput, NLayout, NLayoutContent, NLayoutSider, NScrollbar } from 'naive-ui'
import { Icon } from '@iconify/vue'
import { listen } from '@tauri-apps/api/event'
import { STATUS_CHANGE, StatusChangePayloadType } from '@/const/listen'
import { commit, currentRemoteBranch, getBranches, getChangedFiles, getCurrentBranch, getStagedFiles, pull, push, switchBranch } from '@/utils/command'
import { Branch, ChangedFile } from '@/types'

import ChangedFileWidget from './components/changed-file-widget.vue'
import { GitUtilsErrorCode } from '@/enum/error'
import { RemoteUserPwdDialogProps } from '../remote-user-pwd-dialog/types'
import { createRemoteUserPwdDialog } from '../remote-user-pwd-dialog'

const userAuthorRetry = (zindex: number, param: RemoteUserPwdDialogProps, cb?: (res) => void, ecb?: (e) => void) => {
  createRemoteUserPwdDialog(param).then(async (res) => {
    cb && cb(res)
  }).catch((e) => {
    if (ecb) {
      ecb(e) 
    } else {
     throw e 
    }
  })
}

const className = '__source__control__container'

const branchKeyPrefix = '__BRANCH__:'

export function useSourceControlDialog(props: SourceConterolDialogProps) {
  const commitMsg = ref('')
  const changedFiles = ref<ChangedFile[]>([])
  const stagedFiles = ref<ChangedFile[]>([])
  const branches = ref<Branch[]>([])
  const currentBranch = ref<Branch>()
  const currentRemoteBranchRef = ref<Branch | undefined>()
  let computedOption: ComputedRef<any[]> | undefined
  let dialogActions: any

  const options: DialogOptions = {
    containerName: className,
    buttonBox: 'custom',
    title: '源码控制',
    subTitle: props.repo.alias,
  }

  const beforeOk = (): boolean => {
    console.log(commitMsg.value)
    return true
  }

  const commitAction = () => {
    commit(props.repo.path, commitMsg.value, undefined).then((res) => {
      commitMsg.value = ''
      window.$message.success('提交成功')
    })
  }

  const pushAction = () => {
    if (currentRemoteBranchRef.value) {
      // 获取remoteRef
      const remoteRef = currentRemoteBranchRef.value.reference.split('/')[0]
      push(props.repo.path, remoteRef, currentBranch.value!.name, undefined).then((res) => {
        window.$message.success('推送成功')
      }).catch((e) => {
        // 如果是需要用户名密码的错误，弹出对话框
        if (e.code == GitUtilsErrorCode.PushNeedNameAndPassword) {
          window.$message.error('请输入远程仓库的用户名和密码')
          userAuthorRetry(
            dialogActions.zIndex.value + 1, 
            {subtitle: props.repo.alias}, 
            (res) => {
              if (res) {
                push(props.repo.path, remoteRef, currentBranch.value!.name, [res.username, res.password]).then((res) => {
                  window.$message.success('推送成功')
                }).catch((e) => {
                  if (e.code == GitUtilsErrorCode.RemoteHeadHasNotInLocal) {
                    window.$message.error('远程仓库的HEAD不在本地，请先拉取')
                  }
                  else {
                    window.$message.error(`推送失败 ${e.message}`) 
                  }
                })
              }
            }
          )
        } else {
          window.$message.error(`推送失败 ${e.message}`)
        }
      })
    }
  }

  const pullAction = () => {
    if (currentRemoteBranchRef.value) {
      const remoteRef = currentRemoteBranchRef.value.reference.split('/')[0]
      pull(props.repo.path, remoteRef, currentBranch.value!.name, undefined).then((res) => {
        window.$message.success('拉取成功')
      }).catch(e => {
        console.log(e)
        if (e.code == GitUtilsErrorCode.PushNeedNameAndPassword) {
          window.$message.error('请输入远程仓库的用户名和密码')
          userAuthorRetry(
            dialogActions.zIndex.value + 1, 
            {subtitle: props.repo.alias}, 
            (res) => {
              if (res) {
                pull(props.repo.path, remoteRef, currentBranch.value!.name, [res.username, res.password]).then((res) => {
                  window.$message.success('拉取成功')
                }).catch((e) => {
                  if (e.code == GitUtilsErrorCode.HasConflicts) {
                    window.$message.error('拉取后存在冲突，请手动解决')
                  }
                  else {
                    window.$message.error(`拉取失败 ${e.message}`) 
                  }
                })
              }
            }
          )
        } else if (e.message == GitUtilsErrorCode.CommitBeforePullWouldBeOverwrittenByMerge) {
          window.$message.error('拉取文件与本地存在冲突，请提交并在拉取后手动解决')
        }
      })
    } 
  }

  const switchBranchAction = (branch_name: string) => {
    const branchName = branch_name.replace(branchKeyPrefix, '')
    if (branchName == currentBranch.value?.reference) {
      window.$message.error('当前分支已经是' + branchName)
    } 
    else {
      const branch = branches.value.find((branch) => branch.reference == branchName)
      branch && switchBranch(props.repo.path, branch).then((res) => {
        window.$message.success('切换分支成功') 
      }).catch((e) => {
        window.$message.error(e.message)
      })
    } 
  }

  const customFooter = (): Component => {
    const computedOption = computed(() => {
      const options: any[] = []
      options.push({
        label: '切换分支',
        key: 'switch-branch',
        children: branches.value.map((branch) => {
          return {
            label: branch.name,
            key: branchKeyPrefix + branch.reference
          }
        })
      })
      if (currentRemoteBranchRef.value) {
        options.push({
          label: '推送',
          key: 'push' 
        },
        {
          label: '拉取',
          key: 'pull'
        })
      }
      return options
    })
    const menuProps = (e) => {
      if ( e && e.key === 'switch-branch') {
        return {
          style: {
            'max-height': '200px'
          }
        }
      }
    }
    const handleSelect = (e) => {
      if (e === 'push') {
        pushAction()
      }
      else if (e === 'pull') {
        pullAction()
      }
      else if (e.startsWith(branchKeyPrefix)) {
        switchBranchAction(e)
      }
    }
    return () => (
      <div class="flex justify-between">
      <NDropdown trigger='click' 
        overlap 
        scrollable 
        options={computedOption?.value} 
        menu-props={menuProps}
        on-select={handleSelect}
        arrow-wrapper-style={{
          'overflow-x': 'hidden'
        }}>
        <NButton type="primary">
          更多
        </NButton>
      </NDropdown>
      <NButton onClick={dialogActions.close}>
        关闭
      </NButton>
    </div>
    )
  }
  
  const content = (): Component => {
    return defineComponent({
      name: 'SourceControlDialog',
      setup() {
        const commitOptions = [
          {
            label: '提交',
            key: 'commit'
          },
          {
            label: '修改提交',
            key: 'commit-amend'
          },
          {
            label: '提交并推送',
            key: 'commit-push'
          },
          {
            label: '提交并同步',
            key: 'commit-sync'
          }
        ]
        const flush = () => {
          getChangedFiles(props.repo.path).then((files) => {
            changedFiles.value = files
          })
          getStagedFiles(props.repo.path).then((files) => {
            stagedFiles.value = files
          })
          getBranches(props.repo.path).then((branchList) => {
            console.log(branchList)
            branches.value = branchList 
          })
          currentRemoteBranch(props.repo.path).then((branch) => {
            currentRemoteBranchRef.value = branch
          }).catch((e) => {
            currentRemoteBranchRef.value = undefined
          })
          getCurrentBranch(props.repo.path).then((branch) => {
            currentBranch.value = branch 
          })
        }
        onBeforeMount(() => {
          flush() 
        })
        const unsubscrib = listen<StatusChangePayloadType>(STATUS_CHANGE, (event) => {
          if (event.payload.path === props.repo.path) {
            flush()
          }
        })

        onBeforeUnmount(() => {
          unsubscrib.then((unsub) => {
            unsub()
          }) 
        })
        return () => (
          <div class='grid grid-cols-1 grid-rows-[auto_1fr] max-h-full'>
            {/* 头部commit书写区域 */}
            <div class='flex flex-col gap-2 mb-1'>
              <NInput maxlength="200" v-model:value={commitMsg.value} autosize={{minRows: 1, maxRows: 3}} type="textarea" placeholder="请输入提交内容">
              </NInput>
              <div class='flex h-[30px] bg-[#0078d4] text-white rounded-sm'>
                <NButton class='flex-1' color='#026ec1' text textColor={'white'} onClick={commitAction}>
                  {`提交(${currentBranch.value?.name})`}
                </NButton>
                <NDropdown text trigger='click' options={commitOptions}>
                  <div class='flex justify-center items-center hover:bg-[#026ec1]'>
                    <div class="h-[24px] border-l-[1px] px-1">
                      <Icon icon="iconamoon:arrow-down-2" width="24" height="24" />
                    </div>
                  </div>
                </NDropdown>
              </div>
            </div>
            {/* 变更的文件 */}
            <NLayout 
              nativeScrollbar={false}>
              <NLayoutContent>
                <div>
                  {
                    stagedFiles.value.map((file) => {
                      return <ChangedFileWidget repo={props.repo} key={file.path} file={file} type='staged'/>
                    })
                  }
                </div>
                {/* divider */}
                <div class='flex justify-center items-center
                before:border-b before:flex-1 before:block
                after:border-b after:flex-1 after:block
                after:h-full'>
                  {
                    stagedFiles.value.length > 0 && <div class='px-1 text-xs text-gray-600'>暂存↑</div>
                  }
                  {
                    changedFiles.value.length > 0 && <div class='px-1 text-xs text-gray-600'>修改↓</div>
                  }
                </div>
                <div class="flex-1">
                  {
                    changedFiles.value.map((file) => {
                      return <ChangedFileWidget repo={props.repo} key={file.path} file={file} type='changed'/> 
                    })
                  } 
                </div>
              </NLayoutContent>
            </NLayout>
          </div>
        )
      }
    })
  }

  const callbacks: DialogCallbacks = {
    beforeOk,
    customFooter,
    content
  }

  dialogActions = useAbstractDialog(options, callbacks)
  
  return {
    ...dialogActions,
    commitMsg,
    changedFiles,
    stagedFiles,
    branches,
    currentBranch,
    currentRemoteBranch: currentRemoteBranchRef,
    commitAction,
    pushAction,
    pullAction,
    switchBranchAction
  }
}

export function createSourceControlDialog(props: SourceConterolDialogProps): Promise<undefined> {
  const dialog = useSourceControlDialog(props)
  return dialog.showDialog()
}