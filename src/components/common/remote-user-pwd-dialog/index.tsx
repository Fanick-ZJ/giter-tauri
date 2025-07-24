import { Component, defineComponent, ref } from "vue";
import { useAbstractDialog, DialogOptions, DialogCallbacks } from "../abstract-dialog";
import { DialogReturnType, RemoteUserPwdDialogProps } from "./types";
import { NInput } from "naive-ui";

export function useRemoteUserPwdDialog(props: RemoteUserPwdDialogProps) {
  const username = ref('');
  const password = ref('');
  let dialogActions: any;
  
  const options: DialogOptions = {
    containerName: '__remote__user__pwd__container',
    buttonBox: 'ok-cancel',
    title: '远程仓库验证',
    height: '200px',
    width: '300px'
  };
  
  const beforeOk = (): void => {
    dialogActions.setReturnData({
      username: username.value,
      password: password.value
    })
  }

  const content = (): Component => {
    return defineComponent({
      name: 'RemoteUserPwdDialog',
      setup() {
        return () => (
          <div>
            <div class="flex flex-col gap-2 mb-1">
              <NInput clearable v-model:value={username.value} placeholder="请输入用户名" />
              <NInput clearable type='password' v-model:value={password.value} placeholder="请输入密码" />
            </div>
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
    username,
    password
  }
}

export function createRemoteUserPwdDialog(props: RemoteUserPwdDialogProps): Promise<DialogReturnType> {
  const dialog = useRemoteUserPwdDialog(props)
  return dialog.showDialog()
}