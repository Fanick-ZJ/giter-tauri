import { router } from '@/router';
import { createPinia } from 'pinia'
import { App } from 'vue';
import { useRepoStore } from './modules/repo';

export async function setupStore(app: App) {
  const store = createPinia()
  app.use(router)
  app.use(store)

  // 根据当前的路由，判断是不是要重置路由，使用浏览器的方法判断
  console.log(window.location.pathname)
  if (window.location.pathname.startsWith('/main') 
    || window.location.pathname == '/') {
    // 重置路由，重新初始化仓库
    router.replace('/')
    await router.isReady()
    useRepoStore().init_repo()
  }
}