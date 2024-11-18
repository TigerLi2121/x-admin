<template>
  <div>
    <t-card class="list-card-container">
      <t-row class="table-bar" justify="space-between">
        <t-col :span="3">
          <div>
            <!-- <span v-has="'log:add'">
              <t-button @click="add"> 新增 </t-button>
            </span>
            <div v-has="'log:delete'" class="delete-btn">
              <t-button variant="base" theme="danger" :disabled="!selectedRowKeys.length" @click="delBatch"
                >删除</t-button
              >
              <p v-if="!!selectedRowKeys.length" class="selected-count">已选{{ selectedRowKeys.length }}项</p>
            </div> -->
          </div>
        </t-col>
        <t-col :span="9">
          <t-row>
            <t-col :flex="1">
              <div style="width: 360px">
                <t-input v-model="query.q" placeholder="请输入操作" type="text" />
              </div>
            </t-col>
            <t-col :flex="1">
              <t-button @click="list(pagination)">查询</t-button>
            </t-col>
          </t-row>
        </t-col>
      </t-row>
      <t-table :columns="columns" :data="dList" :hover="true" :loading="isLoading" :pagination="pagination"
        :selected-row-keys="selectedRowKeys" row-key="id" @page-change="(pi) => list(pi)" @select-change="selectChange">
        <!-- <template #op="{ row }">
          <a v-has="'log:update'" class="t-button-link" @click="update(row)">修改</a>
          <span v-has="'log:delete'">
            <t-popconfirm theme="warning" content="确认删除吗" @confirm="del([row.id])">
              <a class="t-button-link">删除</a>
            </t-popconfirm>
          </span>
        </template> -->
      </t-table>

      <t-dialog v-model:visible="dialogVisible" :footer="null" :header="d.id ? '修改' : '新增'" attach="body">
        <template #body>
          <t-form ref="form" :data="d" reset-type="initial" @submit="onSubmit">
            <t-form-item label="id" name="id">
              <t-input v-model="d.id" placeholder="请输入id"></t-input>
            </t-form-item>
            <t-form-item label="用户名" name="username">
              <t-input v-model="d.username" placeholder="请输入用户名"></t-input>
            </t-form-item>
            <t-form-item label="用户操作" name="operation">
              <t-input v-model="d.operation" placeholder="请输入用户操作"></t-input>
            </t-form-item>
            <t-form-item label="请求方法" name="method">
              <t-input v-model="d.method" placeholder="请输入请求方法"></t-input>
            </t-form-item>
            <t-form-item label="请求参数" name="params">
              <t-input v-model="d.params" placeholder="请输入请求参数"></t-input>
            </t-form-item>
            <t-form-item label="执行时长(毫秒)" name="time">
              <t-input v-model="d.time" placeholder="请输入执行时长(毫秒)"></t-input>
            </t-form-item>
            <t-form-item label="IP地址" name="ip">
              <t-input v-model="d.ip" placeholder="请输入IP地址"></t-input>
            </t-form-item>
            <t-form-item label="创建时间" name="created_at">
              <t-input v-model="d.created_at" placeholder="请输入创建时间"></t-input>
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
<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { DialogPlugin, MessagePlugin } from 'tdesign-vue-next';
import request from '@/utils/request';
// 列表数据
const columns: any = [
  // { colKey: 'row-select', type: 'multiple', width: 64, fixed: 'left' },
  { colKey: 'id', title: 'id', ellipsis: true, width: 80 },
  { colKey: 'username', title: '用户名', ellipsis: true, width: 80 },
  { colKey: 'operation', title: '用户操作', ellipsis: true, width: 120 },
  { colKey: 'method', title: '请求方法', ellipsis: true, width: 120 },
  { colKey: 'params', title: '请求参数', ellipsis: true, width: 220 },
  { colKey: 'time', title: '执行时长(ms)', ellipsis: true, width: 120 },
  { colKey: 'ip', title: 'IP地址', ellipsis: true, width: 180 },
  { colKey: 'created_at', title: '创建时间', ellipsis: true, width: 180 },
  // { colKey: 'op', title: '操作', align: 'center', fixed: 'right', width: 160 },
];
const form: any = ref({});
const dList = ref([]);
const d: any = ref({});
const dialogVisible = ref(false);
const query: any = ref({});
// 分页数据
const pagination = ref({
  current: 1,
  pageSize: 10,
  total: 0,
});
const isLoading = ref(false);
const list: any = async (page = pagination.value) => {
  isLoading.value = true;
  try {
    const res: any = await request.get('/api/sys_log', {
      params: { ...query.value, page: page.current, limit: page.pageSize },
    });
    if (res.data) {
      dList.value = res.data;
      pagination.value = { ...page, total: res.total };
    }
  } catch (e) {
    console.log(e);
  } finally {
    isLoading.value = false;
  }
};
// 初始数据
onMounted(async () => {
  await list();
});
// 新增
const add = () => {
  dialogVisible.value = true;
  form.value.reset();
  form.value.clearValidate();
};
// 修改
const update = (row) => {
  dialogVisible.value = true;
  d.value = { ...row };
};
// 提交
const onSubmit = async ({ validateResult, firstError, e }) => {
  e.preventDefault();
  if (validateResult === true) {
    const res: any = await request.post('/api/sys_log', d.value);
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
  const res: any = await request.delete('/api/sys_log', { data: ids });
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
