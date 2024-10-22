import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Layout from '@/layouts/index.vue';

// 默认路由
const defaultRouterList: Array<RouteRecordRaw> = [
  {
    path: '/login',
    name: 'login',
    component: () => import('@/pages/login.vue'),
  },
  {
    path: '/',
    redirect: '/home',
  },
  {
    path: '/home',
    component: Layout,
    redirect: '/home/index',
    name: 'homeParent',
    meta: { title: '首页' },
    children: [
      {
        path: 'index',
        name: 'home',
        component: () => import('@/pages/home.vue'),
        meta: { title: '首页' },
      },
    ],
  },
  {
    path: '/personal',
    name: 'personalParent',
    component: Layout,
    redirect: '/personal/personal',
    meta: { title: '个人中心' },
    children: [
      {
        path: 'personal',
        name: 'personal',
        component: () => import('@/pages/user_info.vue'),
        meta: { title: '个人中心' },
      },
    ],
  },
  {
    path: '/404',
    name: '404Parent',
    component: Layout,
    redirect: '/404/index',
    meta: { title: '404' },
    children: [
      {
        path: 'index',
        name: '404',
        component: () => import('@/pages/404.vue'),
        meta: { title: '404' },
      },
    ],
  },
];

export const getActive = (maxLevel = 3): string => {
  // 非组件内调用必须通过Router实例获取当前路由
  const route = router.currentRoute.value;

  if (!route.path) {
    return '';
  }

  return route.path
    .split('/')
    .filter((_item: string, index: number) => index <= maxLevel && index > 0)
    .map((item: string) => `/${item}`)
    .join('');
};

const router = createRouter({
  history: createWebHistory(import.meta.env.VITE_BASE_URL),
  routes: defaultRouterList,
  scrollBehavior() {
    return {
      el: '#app',
      top: 0,
      behavior: 'smooth',
    };
  },
});

export default router;
