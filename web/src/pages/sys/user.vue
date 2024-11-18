<template>
  <div>
    <t-card class="list-card-container">
      <t-row justify="space-between" class="table-bar">
        <t-col :span="3">
          <div>
            <span v-has="'user:add'">
              <t-button @click="add"> 新增 </t-button>
            </span>
            <div v-has="'user:delete'" class="delete-btn">
              <t-button variant="base" theme="danger" :disabled="!selectedRowKeys.length"
                @click="delBatch">删除</t-button>
              <p v-if="!!selectedRowKeys.length" class="selected-count">已选{{ selectedRowKeys.length }}项</p>
            </div>
          </div>
        </t-col>
        <t-col :span="9">
          <t-row>
            <t-col :flex="1">
              <div style="width: 360px">
                <t-input v-model="query.q" placeholder="请输入昵称或账号" type="text" />
              </div>
            </t-col>
            <t-col :flex="1">
              <t-button @click="list(pagination)">查询</t-button>
            </t-col>
          </t-row>
        </t-col>
      </t-row>
      <t-table row-key="id" :columns="columns" :data="ds" :hover="true" :pagination="pagination"
        :selected-row-keys="selectedRowKeys" :loading="isLoading" @page-change="(pi) => list(pi)"
        @select-change="selectChange">
        <template #status="{ row }">
          <t-tag v-if="row.status === 1" theme="success" variant="light"> 正常 </t-tag>
          <t-tag v-if="row.status === 0" theme="warning" variant="light"> 禁用 </t-tag>
        </template>

        <template #op="{ row }">
          <t-space>
            <span v-has="'user:update'">
              <t-link theme="primary" @click="update(row)">修改</t-link>
            </span>
            <span v-has="'user:delete'">
              <t-popconfirm theme="warning" content="确认删除吗" @confirm="del([row.id])">
                <t-link theme="danger">删除</t-link>
              </t-popconfirm>
            </span>
          </t-space>
        </template>
      </t-table>

      <t-dialog v-model:visible="dialogVisible" attach="body" :header="d.id ? '修改' : '新增'" :footer="null">
        <template #body>
          <t-form ref="form" :data="d" reset-type="initial" :rules="rules" @submit="onSubmit">
            <t-form-item label="账号" name="username">
              <t-input v-model="d.username" placeholder="请输入账号"></t-input>
            </t-form-item>
            <t-form-item label="密码" name="password">
              <t-input v-model="d.password" placeholder="请输入密码"></t-input>
            </t-form-item>
            <t-form-item label="手机号" name="mobile">
              <t-input v-model="d.mobile" placeholder="请输入手机号"></t-input>
            </t-form-item>
            <t-form-item label="邮箱" name="email">
              <t-input v-model="d.email" placeholder="请输入邮箱"></t-input>
            </t-form-item>
            <t-form-item label="角色" name="role_ids">
              <t-checkbox-group v-model="d.role_ids" :options="roleSelect"></t-checkbox-group>
            </t-form-item>
            <t-form-item label="状态" name="status">
              <t-radio-group v-model="d.status">
                <t-radio :value="1">正常</t-radio>
                <t-radio :value="0">禁用</t-radio>
              </t-radio-group>
            </t-form-item>
            <t-form-item style="padding-top: 8px">
              <t-button theme="primary" type="submit">提交</t-button>
            </t-form-item>
          </t-form>
        </template>
      </t-dialog>
    </t-card>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { MessagePlugin, DialogPlugin } from 'tdesign-vue-next';

import request from '@/utils/request';
// 列表数据
const columns: any = [
  { colKey: 'row-select', type: 'multiple', width: 64, fixed: 'left' },
  { colKey: 'id', title: 'id', width: 64, ellipsis: true, fixed: 'left' },
  { colKey: 'username', title: '账号', width: 150, ellipsis: true, fixed: 'left' },
  { colKey: 'mobile', title: '手机号', width: 150, ellipsis: true },
  { colKey: 'email', title: '邮箱', width: 150, ellipsis: true },
  { colKey: 'status', title: '状态', width: 100, ellipsis: true },
  { colKey: 'created_at', title: '创建时间', width: 180, ellipsis: true },
  { colKey: 'updated_at', title: '修改时间', width: 180, ellipsis: true },
  { colKey: 'op', title: '操作', align: 'center', fixed: 'right', width: 160 },
];
const form: any = ref({});
const ds = ref([]);
const roleSelect = ref([]);
const d: any = ref({ role_ids: [] });
const dialogVisible = ref(false);
// 分页数据
const pagination = ref({
  current: 1,
  pageSize: 10,
  total: 0,
});
// 搜索数据
const query = ref({
  q: '',
});
const isLoading = ref(false);
const list = async (page: any = pagination.value) => {
  isLoading.value = true;
  try {
    const res: any = await request.get('/api/user', {
      params: { page: page.current, limit: page.pageSize, ...query.value },
    });
    if (res.data) {
      ds.value = res.data;
      pagination.value = { ...page, total: res.total };
    }
  } catch (e) {
    console.log(e);
  } finally {
    isLoading.value = false;
  }
};
// 角色选择数据
const getRoleSelect = async () => {
  const res: any = await request.get('/api/role?page=1&limit=10000');
  if (res.data) {
    return res.data.map((e) => ({ value: e.id, label: e.name }));
  }
  return [];
};
// 初始数据
onMounted(async () => {
  await list();
  roleSelect.value = await getRoleSelect();
});
// 新增
const add = () => {
  dialogVisible.value = true;
  // 重置参数
  form.value.reset();
  // 清空校验结果
  form.value.clearValidate();
  d.value = { role_ids: [], status: 1 };
};
// 修改
const update = (row) => {
  dialogVisible.value = true;
  d.value = { ...row };
};
// 表单验证
const rules: any = {
  username: [{ required: true, type: 'error' }],
  email: [{ required: true, type: 'error' }],
  role_ids: [{ required: true, message: '角色必选', type: 'error' }],
};
// 提交
const onSubmit = async ({ validateResult, firstError, e }) => {
  e.preventDefault();
  if (validateResult === true) {
    const res: any = await request.post('/api/user', d.value);
    if (res.code === 0) {
      MessagePlugin.success('处理成功');
      dialogVisible.value = false;
      await list();
    } else {
      console.log(firstError);
    }
  }
};
// 删除数据
const del = async (ids) => {
  const res: any = await request.delete('/api/user', { data: ids });
  if (res.code === 0) {
    MessagePlugin.success('删除成功');
    await list();
  }
};
// 选中数据
const selectedRowKeys = ref([]);
const selectChange = (val: number[]) => {
  selectedRowKeys.value = val;
};
// 批量删除
const delBatch = () => {
  const dialog = DialogPlugin({
    theme: 'warning',
    header: '确认是否删除？',
    onConfirm: () => {
      del(selectedRowKeys.value);
      dialog.hide();
    },
  });
};
</script>

<style lang="less" scoped>
.delete-btn {
  display: inline-block;
  margin-left: 10px;

  .selected-count {
    display: inline-block;
    margin-left: 8px;
    color: var(--td-text-color-secondary);
  }
}
</style>
