import { defineStore } from 'pinia';

import { usePermissionStore } from '@/store';
import request from '@/utils/request';

export const useUserStore = defineStore('user', {
  state: () => ({
    token: '',
    userInfo: {
      username: '',
      email: '',
      mobile: '',
      menus: [],
    },
  }),
  actions: {
    async login(user: any) {
      const res: any = await request.post('/api/login', user);
      if (res.data) {
        this.token = res.data.token;
        localStorage.setItem('x-token', this.token);
      }
      return res;
    },
    async getUserInfo() {
      this.token = localStorage.getItem('x-token');
      const res = await request.get('/api/user/current');
      if (res.data) {
        this.userInfo = res.data;
      }
      return res;
    },
    async logout() {
      this.token = '';
      localStorage.clear();
    },
  },
  persist: {
    afterRestore: () => {
      const permissionStore = usePermissionStore();
      permissionStore.initRoutes();
    },
  },
});
