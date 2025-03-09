import { Component, defineComponent, ref } from "vue";
import { AbstractDialog } from "../abstract-dialog";
import { DialogReturnType, RemoteUserPwdDialogProps } from "./types";
import { NInput } from "naive-ui";

export class RemoteUserPwdDialog extends AbstractDialog<DialogReturnType> {
  private props: RemoteUserPwdDialogProps;
  private username = ref('');
  private password = ref('');
  constructor(props: RemoteUserPwdDialogProps) {
    super({
      containerName: '__remote__user__pwd__container',
      buttonBox: 'ok-cancel',
      title: '远程仓库验证',
      height: '200px',
      width: '300px'
    });
    this.props = props;
  }
  
  public beforeOk(): void {
    this.setReturnData({
      username: this.username.value,
      password: this.password.value
    })
  }

  public content(): Component {
    const self = this;
    return defineComponent({
      name: 'RemoteUserPwdDialog',
      setup() {
        return () => (
          <div>
            <div class="flex flex-col gap-2 mb-1">
              <NInput clearable v-model:value={self.username.value} placeholder="请输入用户名" />
              <NInput clearable type='password' v-model:value={self.password.value} placeholder="请输入密码" />
            </div>
          </div> 
        )
      } 
    })
  }
} 