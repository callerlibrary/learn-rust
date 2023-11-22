import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
  {
    path: "/",
    redirect: "/json",
  },
  {
    path: "/json",
    redirect: "/json/format",
    children: [
      {
        path: "format",
        name: "format",
        component: () => import("@/views/json/format/index.vue"),
      },
    ],
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
