import { createMemoryHistory, createRouter, createWebHistory } from "vue-router";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/main" 
    },
    {
      name: 'main',
      path: '/main',
      component: () => import('@/views/main/index.vue'),
      children: [
        {
          name: "commit",
          path: "/main/commit/:id",
          component: () => import("@/views/main/commit/index.vue")
        },
        {
          name: "contribution",
          path: "/main/contribution/:path",
          component: () => import("@/views/main/contribution/index.vue") 
        }   
      ]
    },
    {
      name: 'file-history',
      path: '/file_history',
      component: () => import('@/views/file-history/index.vue')
    }
  ]
})