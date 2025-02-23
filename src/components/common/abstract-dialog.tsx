import { createSingletonComponent, InstanceManager, InstanceProps } from "@/utils/tool";
import { NButton, NCard } from "naive-ui";
import { Component, defineComponent, h, nextTick, ref, Ref } from "vue";

type DialogOptions = {
  buttonBox?: 'cancel' | 'ok' | 'ok-cancel' | 'custom',
  containerName: string,
  title?: String | Ref<String>,
  subTitle?: String | Ref<String>,
  width?: string,
  height?: string
}

// R 组件返回类型
// P 组件props类型
export class AbstractDialog<R> {
  private containerName;
  private _promise?: Promise<R>;  // 界面显示时的promise
  private _resolve: Function = () => undefined; // 界面显示时的resolve
  private _reject: Function = () => undefined;  // 界面显示时的reject
  private _show: Ref<boolean>;  // 界面是否显示
  private title: String | Ref<String>;  // 界面标题
  private subTitle: String | Ref<String>; // 界面副标题
  private comp?: InstanceManager<Component> // 界面实例
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
    let _this = this;
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
      _this._resolve = resolve;
      _this._reject = reject;
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
  private ok() {
    this.beforeOk();
    this._resolve(this.returnData);
    if (this.comp) {
      this.comp.unmount()
      this.comp = undefined
    }
    this._show.value = false; 
  }
  
  public beforeClose() {}

  private async close () {
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

  public header(): Component | undefined {
    const _this = this;
    return () => (
      <div class='flex w-max gap-2 overflow-x-hidden'>
        <div class='flex-1'>
          <div class='text-lg font-bold'>
            {_this.title}
          </div>
        </div>
        <div class='text-sm flex items-end'>
          {_this.subTitle}
        </div>
      </div> 
    )
  }

  public customFooter(): Component | undefined {
    return undefined 
  }

  private footer(): Component | undefined {
    const _this = this;
    if (this.buttonBox === 'cancel') {
      return () => (
        <div class="flex justify-end">
          <NButton onClick={_this.close.bind(_this)}>
            关闭
          </NButton>
        </div>
      )
    }
    else if (this.buttonBox === 'ok') {
      return () => (
        <div class="flex justify-end">
          <NButton onClick={_this.ok.bind(_this)}>
            确定
          </NButton>
        </div>
      ) 
    }
    else if (this.buttonBox === 'ok-cancel') {
      return () => (
        <div class="flex justify-end">
          <NButton onClick={_this.ok.bind(_this)}>
            确定
          </NButton>
          <NButton onClick={_this.close.bind(_this)}>
            取消
          </NButton>
        </div>
      ) 
    } else {
      return _this.customFooter()
    }
  }

  private component (): Component {
    const _this = this;
    return defineComponent({
      name: 'AbstractDialog',
      setup() {
        const header = _this.header()
        const footer = _this.footer()
        const slots = {
          default: () => h(_this.content()),
          header: () => header ? h(header!) : undefined,
          footer: () => footer ? h(footer!) : undefined
        }
        const style = {
          width: _this.width,
          height: _this.height 
        }
        return () => (
        <>
        {
          _this._show.value ? (
            <div class='w-screen h-screen bg-slate-400/50 flex items-center justify-center fixed top-0 left-0 z-[3]'
              onMousemove={_this.mouseMove.bind(_this)}>
              <NCard style={style}
                 closable 
                 onClose={_this.close.bind(_this)} 
                 v-slots={slots}
                 headerStyle={{'overflow-x': 'hidden', 'overflow-y': 'hidden'}}>
              </NCard>
            </div>
          ) : null
        }
        </>
        ) 
      }
    })
  }
}