import { router } from '@/router';
import { createPinia } from 'pinia'
import { App } from 'vue';
import { useRepoStore } from './modules/repo';

export function setupStore(app: App) {
  const pathname = window.location.pathname
  if (pathname.startsWith('/main') && pathname !== '/main') {
    // 重定向到'/'
    window.location.href = '/'

  }
  const store = createPinia()
  app.use(router)
  app.use(store)
}