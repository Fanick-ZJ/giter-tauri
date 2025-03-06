// 使用组件，通过函数调用的方式，将组件附着在root上
import {Component, computed, defineComponent, nextTick, onBeforeMount, onBeforeUnmount, ref, watch } from 'vue'
import { SourceConterolDialogProps } from './types'
import { AbstractDialog } from '../abstract-dialog'
import { NButton, NDivider, NDropdown, NInput } from 'naive-ui'
import { Icon } from '@iconify/vue/dist/iconify.js'
import { listen } from '@tauri-apps/api/event'
import { STATUS_CHANGE, StatusChangePayloadType } from '@/const/listen'
import { commit, currentRemoteBranch, getBranches, getChangedFiles, getCurrentBranch, getStagedFiles, push } from '@/utils/command'
import { Branch, ChangedFile } from '@/types'

import ChangedFileWidget from './components/changed-file-widget.vue'

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
      console.log(res)
    })
  }

  public customFooter(): Component | undefined {
    const self = this;
    const computedOption = computed(() => {
      const options = [] as any
      if (self.currentRemoteBranch.value) {
        options.push({
          label: '推送',
          key: 'push' 
        })
      }
      options.push({
        label: '切换分支',
        key: 'switch-branch',
        children: self.branches.value.map((branch) => {
          return {
            label: branch.name,
            key: branchKeyPrefix + branch.name
          }
        })
      })
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
        if (self.currentRemoteBranch.value) {
          console.log('push')
          // 获取remoteRef
          const remoteRef = self.currentRemoteBranch.value.reference.split('/')[0]
          console.log(self.props.repo.path, remoteRef, this.currentBranch.value!.name)
          push(self.props.repo.path, remoteRef, this.currentBranch.value!.name, undefined).then((res) => {
            window.$message.success('推送成功')
          }).catch((e) => {
            // if ()
            console.log(e)
          })
        }
      }
    }
    return () => (
      <div class="flex justify-between">
      <NDropdown trigger='click' 
        overlap 
        scrollable 
        options={computedOption.value} 
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
          <div>
            {/* 头部commit书写区域 */}
            <div class='flex flex-col gap-2 mb-1'>
              <NInput maxlength="200" v-model:value={self.commitMsg.value} autosize={{minRows: 1, maxRows: 3}} type="textarea" placeholder="请输入提交内容">
              </NInput>
              <div class='flex h-[30px] bg-[#0078d4] text-white rounded-sm'>
                <NButton class='flex-1' color='#026ec1' text textColor={'white'} onClick={self.commit.bind(self)}>
                  提交
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
            <div>
              {/* 变更的文件 */}
              {
                self.stagedFiles.value.map((file) => {
                  return <ChangedFileWidget repo={self.props.repo} key={file.path} file={file} type='staged'/>
                })
              }
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
              {
                self.changedFiles.value.map((file) => {
                  return <ChangedFileWidget repo={self.props.repo} key={file.path} file={file} type='changed'/> 
                })
              }
            </div>
          </div>
        )
      }
    })
  }
}