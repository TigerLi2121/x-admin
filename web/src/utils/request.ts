import axios from 'axios';
import { MessagePlugin } from 'tdesign-vue-next';
import { useUserStore } from '@/store';
import router from '@/router';

const env = import.meta.env.MODE || 'development';

const host = env === 'mock' || env === 'development' ? '' : import.meta.env.VITE_API_URL; // 如果是mock模式 就不配置host 会走本地Mock拦截

const request = axios.create({
  baseURL: host,
  timeout: 10000,
  withCredentials: true,
});

request.interceptors.request.use((config) => {
  const token = localStorage.getItem('x-token');
  if (token) {
    config.headers.Authorization = token;
  }
  return config;
});

request.defaults.timeout = 10000;

request.interceptors.response.use(
  (response) => {
    if (response.data.code !== 0) {
      console.log('resp code:', response.data.code);
      MessagePlugin.error(response.data.msg);
    }
    return response.data;
  },
  (err) => {
    console.log('axios err:', err);
    let code = 0;
    try {
      code = err.response.status;
      if (code) {
        console.log('code:' + code);
        if (code === 401) {
          const userStore = useUserStore();
          userStore.logout().then(() => {
            // 跳转到登录页
            if (router.currentRoute.value.path === '/login') {
              router.push(router.currentRoute.value.fullPath);
            } else {
              router.push(`/login?redirect=${router.currentRoute.value.fullPath}`);
            }
          });
        }
      }
    } catch (e) {
      console.log('request e:' + e);
    }
    return Promise.reject(err);
  },
);

export default request;
