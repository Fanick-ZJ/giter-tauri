// 使用组件，通过函数调用的方式，将组件附着在root上
import {Component, defineComponent, nextTick, onBeforeMount, onBeforeUnmount, ref, watch } from 'vue'
import { SourceConterolDialogProps } from './types'
import { AbstractDialog } from '../abstract-dialog'
import { NButton, NDivider, NDropdown, NInput } from 'naive-ui'
import { Icon } from '@iconify/vue/dist/iconify.js'
import { listen } from '@tauri-apps/api/event'
import { STATUS_CHANGE, StatusChangePayloadType } from '@/const/listen'
import { commit, getChangedFiles, getStagedFiles } from '@/utils/command'
import { ChangedFile } from '@/types'

import ChangedFileWidget from './components/changed-file-widget.vue'

const className = '__source__control__container'

export class SourceControlDialog extends AbstractDialog<undefined> {
  private commitMsg = ref('')
  private props: SourceConterolDialogProps
  constructor(props: SourceConterolDialogProps) {
    super({
      containerName: className,
      buttonBox: 'ok',
      title: '源码控制',
      subTitle: props.repo.alias
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
  
  public content(): Component {
    let _this = this
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
        const changedFiles = ref<ChangedFile[]>([])
        const stagedFiles = ref<ChangedFile[]>([])
        const flush = () => {
          getChangedFiles(_this.props.repo.path).then((files) => {
            changedFiles.value = files
          })
          getStagedFiles(_this.props.repo.path).then((files) => {
            stagedFiles.value = files
          })
        }
        onBeforeMount(() => {
          flush() 
        })
        const unsubscrib = listen<StatusChangePayloadType>(STATUS_CHANGE, (event) => {
          if (event.payload.path === _this.props.repo.path) {
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
            <div class='flex flex-col gap-2'>
              <NInput maxlength="200" v-model:value={_this.commitMsg.value} autosize={{minRows: 1, maxRows: 3}} type="textarea" placeholder="请输入提交内容">
              </NInput>
              <div class='flex h-[30px] bg-[#0078d4] text-white rounded-sm'>
                <NButton class='flex-1' color='#026ec1' text textColor={'white'} onClick={_this.commit.bind(_this)}>
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
                stagedFiles.value.map((file) => {
                  return <ChangedFileWidget repo={_this.props.repo} key={file.path} file={file} type='staged'/>
                })
              }
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
              {
                changedFiles.value.map((file) => {
                  return <ChangedFileWidget repo={_this.props.repo} key={file.path} file={file} type='changed'/> 
                })
              }
            </div>
          </div>
        )
      }
    })
  }
}