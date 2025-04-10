// 使用组件，通过函数调用的方式，将组件附着在root上
import {Component, computed, ComputedRef, defineComponent, nextTick, onBeforeMount, onBeforeUnmount, ref, watch } from 'vue'
import { SourceConterolDialogProps } from './types'
import { AbstractDialog } from '../abstract-dialog'
import { NButton, NDivider, NDropdown, NInput, NLayout, NLayoutContent, NLayoutSider, NScrollbar } from 'naive-ui'
import { Icon } from '@iconify/vue/dist/iconify.js'
import { listen } from '@tauri-apps/api/event'
import { STATUS_CHANGE, StatusChangePayloadType } from '@/const/listen'
import { commit, currentRemoteBranch, getBranches, getChangedFiles, getCurrentBranch, getStagedFiles, pull, push, switchBranch } from '@/utils/command'
import { Branch, ChangedFile } from '@/types'

import ChangedFileWidget from './components/changed-file-widget.vue'
import { GitUtilsErrorCode } from '@/enum/error'
import { RemoteUserPwdDialog } from '../remote-user-pwd-dialog'
import { RemoteUserPwdDialogProps } from '../remote-user-pwd-dialog/types'

const userAuthorRetry = (zindex: number, param: RemoteUserPwdDialogProps, cb?: (res) => void, ecb?: (e) => void) => {
  const dlg = new RemoteUserPwdDialog(param)
  dlg.setZIndex(zindex)
  dlg.show()?.then(async (res) => {
    cb && cb(res)
  }).then((e) => {
    if (ecb) {
      ecb(e) 
    } else {
     throw e 
    }
  })
}

const className = '__source__control__container'

const branchKeyPrefix = '__BRANCH__:'

export class SourceControlDialog extends AbstractDialog<undefined> {
  private commitMsg = ref('')
  private props: SourceConterolDialogProps
  private changedFiles = ref<ChangedFile[]>([])
  private stagedFiles = ref<ChangedFile[]>([])
  private branches = ref<Branch[]>([])
  private currentBranch = ref<Branch>()
  private currentRemoteBranch = ref<Branch | undefined>()
  private computedOption?: ComputedRef<any[]>
  constructor(props: SourceConterolDialogProps) {
    super({
      containerName: className,
      buttonBox: 'custom',
      title: '源码控制',
      subTitle: props.repo.alias,
    })
    this.props = props
  }

  public beforeOk(): void {
    console.log(this.commitMsg.value)
  }

  public commit () {
    commit(this.props.repo.path, this.commitMsg.value, undefined).then((res) => {
      this.commitMsg.value = ''
      window.$message.success('提交成功')
    })
  }

  public push() {
    if (this.currentRemoteBranch.value) {
      // 获取remoteRef
      const remoteRef = this.currentRemoteBranch.value.reference.split('/')[0]
      push(this.props.repo.path, remoteRef, this.currentBranch.value!.name, undefined).then((res) => {
        window.$message.success('推送成功')
      }).catch((e) => {
        // 如果是需要用户名密码的错误，弹出对话框
        if (e.code == GitUtilsErrorCode.PushNeedNameAndPassword) {
          window.$message.error('请输入远程仓库的用户名和密码')
          userAuthorRetry(
            this.zIndex.value + 1, 
            {subtitle: this.props.repo.alias}, 
            (res) => {
              if (res) {
                push(this.props.repo.path, remoteRef, this.currentBranch.value!.name, [res.username, res.password]).then((res) => {
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

  public pull() {
    if (this.currentRemoteBranch.value) {
      const remoteRef = this.currentRemoteBranch.value.reference.split('/')[0]
      pull(this.props.repo.path, remoteRef, this.currentBranch.value!.name, undefined).then((res) => {
        window.$message.success('拉取成功')
      }).catch(e => {
        console.log(e)
        if (e.code == GitUtilsErrorCode.PushNeedNameAndPassword) {
          window.$message.error('请输入远程仓库的用户名和密码')
          userAuthorRetry(
            this.zIndex.value + 1, 
            {subtitle: this.props.repo.alias}, 
            (res) => {
              if (res) {
                pull(this.props.repo.path, remoteRef, this.currentBranch.value!.name, [res.username, res.password]).then((res) => {
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

  public switchBranch (branch_name: string) {
    const branchName = branch_name.replace(branchKeyPrefix, '')
    if (branchName == this.currentBranch.value?.reference) {
      window.$message.error('当前分支已经是' + branchName)
    } 
    else {
      const branch = this.branches.value.find((branch) => branch.reference == branchName)
      branch && switchBranch(this.props.repo.path, branch).then((res) => {
        window.$message.success('切换分支成功') 
      }).catch((e) => {
        window.$message.error(e.message)
      })
    } 
  }

  public customFooter(): Component | undefined {
    const self = this;
    this.computedOption = computed(() => {
      const options: any[] = []
      options.push({
        label: '切换分支',
        key: 'switch-branch',
        children: self.branches.value.map((branch) => {
          return {
            label: branch.name,
            key: branchKeyPrefix + branch.reference
          }
        })
      })
      if (self.currentRemoteBranch.value) {
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
        this.push()
      }
      else if (e === 'pull') {
        this.pull()
      }
      else if (e.startsWith(branchKeyPrefix)) {
        this.switchBranch(e)
      }
    }
    return () => (
      <div class="flex justify-between">
      <NDropdown trigger='click' 
        overlap 
        scrollable 
        options={self.computedOption?.value} 
        menu-props={menuProps}
        on-select={handleSelect}
        arrow-wrapper-style={{
          'overflow-x': 'hidden'
        }}>
        <NButton type="primary">
          更多
        </NButton>
      </NDropdown>
      <NButton onClick={self.close.bind(self)}>
        关闭
      </NButton>
    </div>
    )
  }
  
  public content(): Component {
    let self = this
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
          getChangedFiles(self.props.repo.path).then((files) => {
            self.changedFiles.value = files
          })
          getStagedFiles(self.props.repo.path).then((files) => {
            self.stagedFiles.value = files
          })
          getBranches(self.props.repo.path).then((branches) => {
            console.log(branches)
            self.branches.value = branches 
          })
          currentRemoteBranch(self.props.repo.path).then((branch) => {
            self.currentRemoteBranch.value = branch
          }).catch((e) => {
            self.currentRemoteBranch.value = undefined
          })
          getCurrentBranch(self.props.repo.path).then((branch) => {
            self.currentBranch.value = branch 
          })
        }
        onBeforeMount(() => {
          flush() 
        })
        const unsubscrib = listen<StatusChangePayloadType>(STATUS_CHANGE, (event) => {
          if (event.payload.path === self.props.repo.path) {
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
              <NInput maxlength="200" v-model:value={self.commitMsg.value} autosize={{minRows: 1, maxRows: 3}} type="textarea" placeholder="请输入提交内容">
              </NInput>
              <div class='flex h-[30px] bg-[#0078d4] text-white rounded-sm'>
                <NButton class='flex-1' color='#026ec1' text textColor={'white'} onClick={self.commit.bind(self)}>
                  {`提交(${self.currentBranch.value?.name})`}
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
                    self.stagedFiles.value.map((file) => {
                      return <ChangedFileWidget repo={self.props.repo} key={file.path} file={file} type='staged'/>
                    })
                  }
                </div>
                {/* divider */}
                <div class='flex justify-center items-center
                before:border-b before:flex-1 before:block
                after:border-b after:flex-1 after:block
                after:h-full'>
                  {
                    self.stagedFiles.value.length > 0 && <div class='px-1 text-xs text-gray-600'>暂存↑</div>
                  }
                  {
                    self.changedFiles.value.length > 0 && <div class='px-1 text-xs text-gray-600'>修改↓</div>
                  }
                </div>
                <div class="flex-1">
                  {
                    self.changedFiles.value.map((file) => {
                      return <ChangedFileWidget repo={self.props.repo} key={file.path} file={file} type='changed'/> 
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
}