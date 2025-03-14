import { createSingletonComponent, InstanceManager, InstanceProps } from "@/utils/tool";
import { NButton, NCard, NDialogProvider } from "naive-ui";
import { Component, defineComponent, h, nextTick, ref, Ref } from "vue";

type DialogOptions = {
  buttonBox?: 'cancel' | 'ok' | 'ok-cancel' | 'custom',
  containerName: string,
  title?: string | Ref<string>,
  subTitle?: string | Ref<string>,
  width?: string,
  height?: string
}

// R 组件返回类型
// P 组件props类型
export class AbstractDialog<R> {
  private containerName: string;
  private _promise?: Promise<R>;  // 界面显示时的promise
  private _resolve: Function = () => undefined; // 界面显示时的resolve
  private _reject: Function = () => undefined;  // 界面显示时的reject
  private _show: Ref<boolean>;  // 界面是否显示
  private title: string | Ref<string>;  // 界面标题
  private subTitle: string | Ref<string>; // 界面副标题
  private comp?: InstanceManager<Component> // 界面实例
  protected zIndex = ref(3); // 界面层级
  private buttonBox: 'cancel' | 'ok' | 'ok-cancel' | 'custom'; // 按钮框
  private width: string; // 界面宽度
  private height: string; // 界面高度
  private returnData?: R; // 界面返回数据

  constructor(options: DialogOptions) {
    this.containerName = options.containerName
    this.title = options.title || ''
    this.subTitle = options.subTitle || ''
    this.buttonBox = options.buttonBox || 'ok'
    this.width = options.width || '80%';
    this.height = options.height || '80%';
    this._show = ref(false);
  }

  public show() {
    let self = this;
    try {
      this.comp = createSingletonComponent({
        className: this.containerName,
        component: this.component(),
        props: {}
      })
    } catch (e) {
     window.$message.error('窗口实例已存在')
     return 
    }
    this._promise = new Promise((resolve, reject) => {
      self._resolve = resolve;
      self._reject = reject;
    }); 
    this._show.value = true;
    return this._promise;
  }

  public setReturnData(data: R) {
    this.returnData = data; 
  }

  // OK按钮回调前
  public beforeOk() {}

  // OK按钮回调
  protected ok() {
    this.beforeOk();
    this._resolve(this.returnData);
    if (this.comp) {
      this.comp.unmount()
      this.comp = undefined
    }
    this._show.value = false; 
  }
  
  public beforeClose() {}

  protected async close () {
    // 调用关闭回调
    this.beforeClose()
    this._reject(); 
    this._show.value = false;
    await nextTick()
    // 卸载组件
    this.comp?.unmount();
    this.comp = undefined;
  }

  private mouseMove (e: MouseEvent) {
   e.preventDefault() 
  }

  public content(): Component { 
    return () => <></>
  }

  public setZIndex(zIndex: number) {
    this.zIndex.value = zIndex; 
  }

  public header(): Component | undefined {
    const self = this;
    return () => (
      <div class='flex w-max gap-2 overflow-x-hidden'>
        <div class='flex-1'>
          <div class='text-lg font-bold'>
            {self.title}
          </div>
        </div>
        <div class='text-sm flex items-end'>
          {self.subTitle}
        </div>
      </div> 
    )
  }

  public customFooter(): Component | undefined {
    return undefined 
  }

  private footer(): Component | undefined {
    const self = this;
    if (this.buttonBox === 'cancel') {
      return () => (
        <div class="flex justify-end">
          <NButton onClick={self.close.bind(self)}>
            关闭
          </NButton>
        </div>
      )
    }
    else if (this.buttonBox === 'ok') {
      return () => (
        <div class="flex justify-end">
          <NButton onClick={self.ok.bind(self)}>
            确定
          </NButton>
        </div>
      ) 
    }
    else if (this.buttonBox === 'ok-cancel') {
      return () => (
        <div class="flex justify-end gap-2">
          <NButton onClick={self.ok.bind(self)} type='primary'>
            确定
          </NButton>
          <NButton onClick={self.close.bind(self)}>
            取消
          </NButton>
        </div>
      ) 
    } else {
      console.log('custom footer')
      return self.customFooter()
    }
  }

  private component (): Component {
    const self = this;
    return defineComponent({
      name: 'AbstractDialog',
      setup() {
        const header = self.header()
        const footer = self.footer()
        const slots = {
          default: () => h(self.content()),
          header: () => header ? h(header!) : undefined,
          footer: () => footer ? h(footer!) : undefined
        }
        const style = {
          width: self.width,
          height: self.height 
        }
        return () => (
        <>
        {
          self._show.value ? (
            <div 
              style={{zIndex: self.zIndex.value}}
              class='w-screen h-screen bg-slate-400/50 flex items-center justify-center fixed top-0 left-0'
              onMousemove={self.mouseMove.bind(self)}>
              <NDialogProvider>
                <NCard style={style}
                  closable 
                  onClose={self.close.bind(self)} 
                  v-slots={slots}
                  headerStyle={{'overflow-x': 'hidden', 'overflow-y': 'hidden'}}
                  contentStyle={{'overflow-y': 'hidden'}}>
                </NCard>
              </NDialogProvider>
            </div>
          ) : null
        }
        </>
        ) 
      }
    })
  }
}