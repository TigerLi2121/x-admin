<template>
  <div>
    <t-card class="list-card-container">
      <t-row justify="space-between" class="table-bar">
        <t-col :span="3">
          <div>
            <span v-has="'menu:add'">
              <t-button @click="add"> 新增 </t-button>
            </span>
          </div>
        </t-col>
      </t-row>
      <t-enhanced-table ref="table" row-key="id" :columns="columns" :data="dList" :hover="true" :loading="isLoading"
        :tree="{ treeNodeColumnIndex: 0 }">
        <template #type="{ row }">
          <t-tag v-if="row.type === 1" theme="primary" variant="light"> 目录 </t-tag>
          <t-tag v-if="row.type === 2" theme="success" variant="light"> 菜单 </t-tag>
          <t-tag v-if="row.type === 3" theme="warning" variant="light"> 按钮 </t-tag>
        </template>
        <template #icon="{ row }">
          <t-icon :name="row.icon" />
        </template>
        <template #status="{ row }">
          <t-tag v-if="row.status === 0" theme="danger" variant="light"> 禁用 </t-tag>
          <t-tag v-if="row.status === 1" theme="success" variant="light"> 展示 </t-tag>
          <t-tag v-if="row.status === 2" variant="light"> 隐藏 </t-tag>
        </template>

        <template #op="{ row }">
          <t-space>
            <span v-if="'menu:update'">
              <t-link theme="primary" @click="update(row)">修改</t-link>
            </span>
            <span v-has="'menu:delete'">
              <t-popconfirm theme="warning" content="确认删除吗" @confirm="del([row.id])">
                <t-link theme="danger">删除</t-link>
              </t-popconfirm>
            </span>
          </t-space>
        </template>
      </t-enhanced-table>

      <t-dialog v-model:visible="dialogVisible" attach="body" :header="d.id ? '修改' : '新增'" :footer="null">
        <template #body>
          <t-form ref="form" :data="d" reset-type="initial" :rules="rules" @submit="onSubmit">
            <t-form-item label="类型" name="type">
              <t-radio-group :disabled="d.id != undefined" v-model="d.type" @change="changeType">
                <t-radio :value="1">目录</t-radio>
                <t-radio :value="2">菜单</t-radio>
                <t-radio :value="3">按钮</t-radio>
              </t-radio-group>
            </t-form-item>
            <t-form-item v-if="d.type !== 1" label="目录" name="pid">
              <t-tree-select v-model="d.pid" :data="menuSelect" placeholder="顶级类目" />
            </t-form-item>
            <t-form-item label="名称" name="name">
              <t-input v-model="d.name" />
            </t-form-item>
            <t-form-item v-if="d.type !== 3" label="菜单URL" name="path">
              <t-input v-model="d.path" />
            </t-form-item>
            <t-form-item v-if="d.type == 2" label="组件路径" name="component">
              <t-input v-model="d.component" />
            </t-form-item>
            <t-form-item v-if="d.type === 3" label="权限标识" name="perms">
              <t-input v-model="d.perms" />
            </t-form-item>
            <t-form-item v-if="d.type !== 3" label="菜单图标" name="icon">
              <t-select v-model="d.icon" placeholder="请选择">
                <t-option v-for="item in iconOptions" :key="item.stem" :value="item.stem"
                  :style="{ display: 'inline-block' }">
                  <div>
                    <t-icon :name="item.stem" />
                  </div>
                </t-option>
                <template #valueDisplay><t-icon :name="d.icon" :style="{ marginRight: '8px' }" />{{ d.icon }}</template>
              </t-select>
            </t-form-item>
            <t-form-item label="状态" name="status">
              <t-radio-group v-model="d.status">
                <t-radio :value="1">正常</t-radio>
                <t-radio :value="0">禁用</t-radio>
                <t-radio :value="2">隐藏</t-radio>
              </t-radio-group>
            </t-form-item>
            <t-form-item label="排序" name="sort">
              <t-input-number v-model="d.sort" />
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
import { MessagePlugin } from 'tdesign-vue-next';
import { manifest } from 'tdesign-icons-vue-next';
import request from '@/utils/request';

const dialogVisible = ref(false);
const isLoading = ref(false);
const table: any = ref({});
const form: any = ref({});
const d: any = ref({});
const dList = ref([]);
const menuSelect = ref([{ value: 0, label: '顶级类目', children: [] }]);
const iconOptions = ref(manifest);
// 列数据
const columns: any = [
  { colKey: 'name', title: '名称', width: 200, ellipsis: true, fixed: 'left' },
  { colKey: 'type', title: '类型', width: 100, ellipsis: true, fixed: 'left' },
  { colKey: 'path', title: '菜单URL', width: 160, ellipsis: true },
  { colKey: 'perms', title: '权限标识', width: 160, ellipsis: true },
  { colKey: 'icon', title: '菜单图标', width: 120, ellipsis: true },
  { colKey: 'status', title: '状态', width: 100, ellipsis: true },
  { colKey: 'sort', title: '排序', width: 100, ellipsis: true },
  { colKey: 'created_at', title: '创建时间', width: 180, ellipsis: true },
  { colKey: 'updated_at', title: '修改时间', width: 180, ellipsis: true },
  { colKey: 'op', title: '操作', align: 'center', fixed: 'right', width: 160 },
];
// 获取列表数据
const list = async () => {
  isLoading.value = true;
  try {
    const res: any = await request.get('/x-admin/api/menu');
    if (res.data) {
      dList.value = res.data;
      const catalogList = getCatalog(res.data);
      console.log('catalogList:', catalogList);
      if (catalogList && catalogList.length > 0) {
        menuSelect.value[0].children = catalogList;
      }
    }
  } catch (e) {
    console.log(e);
  } finally {
    isLoading.value = false;
  }
};
// 目录类型
const changeType = (val) => {
  if (val === 2) {
    const catalogList = getCatalog(dList.value);
    console.log('catalogList:', catalogList);
    if (catalogList && catalogList.length > 0) {
      menuSelect.value[0].children = catalogList;
    }
  }
  if (val === 3) {
    const menuList = getMenu(dList.value);
    console.log('menuList:', menuList);
    if (menuList && menuList.length > 0) {
      menuSelect.value[0].children = menuList;
    }
  }
};
// 获取目录数据
const getCatalog = (menus) => {
  return menus
    .filter((item) => item.type === 1)
    .map(({ id: value, name: label, children }) => ({ value, label, children: getCatalog(children) }));
};
// 获取菜单数据
const getMenu = (menus) => {
  return menus
    .filter((item) => item.type !== 3)
    .map(({ id: value, name: label, children }) => ({ value, label, children: getMenu(children) }));
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
  d.value = { pid: 0, type: 1, status: 1, sort: 100 };
};
// 修改
const update = (row) => {
  dialogVisible.value = true;
  d.value = { ...row };
  changeType(d.value.type);
};
// 表单验证
const rules: any = {
  name: [{ required: true, type: 'error' }],
  path: [{ required: true, type: 'error' }],
  icon: [{ required: true, type: 'error' }],
  perms: [{ required: true, type: 'error' }],
  sort: [{ required: true, type: 'error' }],
};
// 提交
const onSubmit = async ({ validateResult, firstError, e }) => {
  e.preventDefault();
  if (validateResult === true) {
    const res: any = await request.post('/x-admin/api/menu', d.value);
    if (res.code === 0) {
      MessagePlugin.success('处理成功');
      dialogVisible.value = false;
      await list();
    }
  } else {
    console.log(firstError);
  }
};
// 删除数据
const del = async (ids) => {
  const res: any = await request.delete('/x-admin/api/menu', { data: ids });
  if (res.code === 0) {
    MessagePlugin.success('删除成功');
    await list();
  }
};
</script>

<style lang="less" scoped></style>
