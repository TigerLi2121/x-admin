import { useUserStore } from '@/store';

export default (Vue: any) => {
  /** 自定义按钮权限指令 */
  Vue.directive('has', {
    mounted(el: any, binding: any) {
      // 获取按钮权限
      if (!Vue.config.globalProperties.$_has(binding.value)) {
        // 移除不匹配的按钮
        el.parentNode.removeChild(el);
      }
    },
  });

  // 检查权限方法
  Vue.config.globalProperties.$_has = (value: any) => {
    const userStore: any = useUserStore();
    const { userInfo } = userStore;
    let isExist = false;
    if (userInfo.perms.includes(value)) {
      isExist = true;
    }
    return isExist;
  };
};
