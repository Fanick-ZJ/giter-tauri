import { createMemoryHistory, createRouter } from "vue-router";

export const router = createRouter({
  history: createMemoryHistory(),
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
    }
  ]
})