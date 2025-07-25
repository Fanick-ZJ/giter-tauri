import { Component, defineComponent, ref } from "vue";
import { useAbstractDialog, DialogOptions, DialogCallbacks, DialogActions } from "@/components/common/abstract-dialog";
import { NEllipsis, NFlex, NLayout, NRadio, NRadioGroup, NScrollbar, NSpace, NText } from "naive-ui";
import { FileHistoryItem } from "@/types";
import dayjs from 'dayjs';

type CompareSelectDialogProps = {
    historyList: FileHistoryItem[]
}

type DialogReturnType = FileHistoryItem

export function useCompareSelectDialog(props: CompareSelectDialogProps) {
  const selectedHistory = ref<string>()
  let dialogActions: DialogActions<DialogReturnType>;
  
  const options: DialogOptions = {
    containerName: '__compare__select__container',
    buttonBox: 'ok-cancel',
    title: '文件对比选择',
    height: '400px',
    width: '500px'
  };
  
  const beforeOk = (): boolean => {
    const selected = props.historyList.find(item => item.file.objectId === selectedHistory.value)
    if (selected == undefined) {
      window.$message.warning("请选择要比较的历史")
      return false
    }
    dialogActions.setReturnData(selected!)
    return true
  }

  const content = (): Component => {
    return defineComponent({
      name: 'CompareSelectDialog',
      setup() {
        return () => (
          <div class="p-4">
            <div class="mb-4">
              <NText>请选择要对比的历史版本：</NText>
            </div>
            <NRadioGroup v-model:value={selectedHistory.value}>
              <NScrollbar style="max-height: 200px;">
                <NFlex vertical>
                  {props.historyList.map((item) => (
                    <NRadio
                      key={item.file.objectId}
                      value={item.file.objectId}
                      class="mb-2"
                    >
                      <NFlex align="center" class="w-full">
                        <div class="flex-1">
                          <div class="font-medium">
                            <NEllipsis linesClamp={3} class="w-[400px]">
                              {{
                                default: () => item.commit.title,
                                tooltip: () => <div>item.commit.title</div>
                              }}
                            </NEllipsis>
                          </div>
                          <div class="text-sm text-gray-500">
                            {item.commit.authorName} · {dayjs(item.commit.datetime).format('YYYY-MM-DD HH:mm:ss')}
                          </div>
                          <div class="text-xs text-gray-400">
                            {item.file.objectId.substring(0, 8)}
                          </div>
                        </div>
                      </NFlex>
                    </NRadio>
                  ))}
                </NFlex>
              </NScrollbar>
            </NRadioGroup>
          </div>
        )
      } 
    })
  }
  
  const callbacks: DialogCallbacks = {
    beforeOk,
    content
  }
  
  dialogActions = useAbstractDialog<DialogReturnType>(options, callbacks)
  
  return {
    ...dialogActions,
    selectedHistory
  }
}

export function createCompareSelectDialog(props: CompareSelectDialogProps): Promise<DialogReturnType> {
  const dialog = useCompareSelectDialog(props)
  return dialog.showDialog()
}