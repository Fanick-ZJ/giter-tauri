import { router } from '@/router';
import { createPinia } from 'pinia'
import { App } from 'vue';
import { useRepoStore } from './modules/repo';

export async function setupStore(app: App) {
  const store = createPinia()
  app.use(router)
  app.use(store)

  await useRepoStore().init_repo()
}