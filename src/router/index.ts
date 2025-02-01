import { createMemoryHistory, createRouter } from "vue-router";

export const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: "/",
      component: () => import("@/views/home/index.vue") 
    },
    {
      path: "/commit",
      component: () => import("@/views/commit/index.vue")
    }
  ]
})