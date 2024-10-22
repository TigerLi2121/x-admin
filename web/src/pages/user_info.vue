<template>
  <t-card class="user-info-list" size="small" title="个人信息">
    <template #option>
      <t-button theme="default" shape="square">
        <t-icon name="edit" size="18" />
      </t-button>
    </template>
    <t-row class="content" justify="space-between">
      <t-col class="contract">
        <div class="contract-title">手机号</div>
        <div class="contract-detail">
          {{ user.mobile }}
        </div>
      </t-col>
      <t-col class="contract">
        <div class="contract-title">邮箱</div>
        <div class="contract-detail">
          {{ user.email }}
        </div>
      </t-col>
    </t-row>
  </t-card>
</template>
<script setup lang="ts">
import { computed } from 'vue'
import { useUserStore } from '@/store'

const userStore = useUserStore()
const user = computed(() => {
  return userStore.userInfo
})
</script>
<style lang="less" scoped>
.user-info-list {
  margin-top: 16px;

  .content {
    width: 90%;
  }

  .contract {
    width: 340px;
    height: 88px;
    border-radius: var(--td-radius-default);
    margin: 8px 0;

    &-title {
      white-space: nowrap;
      text-overflow: ellipsis;
      overflow: hidden;
      line-height: 24px;
      margin: 20px 0 6px;
      font-size: 14px;
      color: var(--td-text-color-placeholder);
    }

    &-detail {
      white-space: nowrap;
      text-overflow: ellipsis;
      overflow: hidden;
      line-height: 40px;
      font-size: 14px;
      color: var(--td-text-color-secondary);
    }
  }

  .contract:last-child {
    margin-bottom: 0;
  }
}
</style>
