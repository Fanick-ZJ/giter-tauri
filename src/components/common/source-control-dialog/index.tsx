// 使用组件，通过函数调用的方式，将组件附着在root上
import {Component, defineComponent, nextTick, ref, watch } from 'vue'
import { SourceConterolDialogProps } from './types'
import { AbstractDialog } from '../abstract-dialog'
import { NButton, NDropdown, NInput } from 'naive-ui'
import { Icon } from '@iconify/vue/dist/iconify.js'

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
    // 
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
        return () => (
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
        )
      }
    })
  }
}