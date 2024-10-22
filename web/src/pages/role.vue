<template>
  <div>
    <t-card class="list-card-container">
      <t-row justify="space-between" class="table-bar">
        <t-col :span="3">
          <div v-has="'role:add'">
            <t-button @click="add"> 新增 </t-button>
          </div>
        </t-col>
        <t-col :span="2">
          <div>
            <p v-if="selectRow" style="display: inline-block; margin-right: 26px">
              {{ selectRow.name }}
            </p>
            <span v-has="'role:update'">
              <t-button :disabled="selectRow.id === undefined" theme="danger" @click="saveMenu"> 保存 </t-button>
            </span>
          </div>
        </t-col>
      </t-row>
      <t-row justify="space-between" class="table-bar">
        <t-col :span="7">
          <t-table
            ref="table"
            row-key="id"
            :columns="columns"
            :data="dList"
            :hover="true"
            :loading="isLoading"
            active-row-type="single"
            :pagination="pagination"
            @page-change="(pi) => list(pi)"
            @row-click="rowClick"
          >
            <template #op="{ row }">
              <t-space>
                <span v-has="'role:update'">
                  <t-link theme="primary" @click="update(row)">修改</t-link>
                </span>
                <span v-has="'role:delete'">
                  <t-popconfirm theme="warning" content="确认删除吗" @confirm="del([row.id])">
                    <t-link theme="danger">删除</t-link>
                  </t-popconfirm>
                </span>
              </t-space>
            </template>
          </t-table>
        </t-col>
        <t-col :span="3">
          <t-tree
            ref="tree"
            v-model="treeData"
            :data="menus"
            hover
            checkable
            line
            expand-all
            value-mode="all"
            :disabled="selectRow.id === undefined"
            @change="treeChange"
          />
        </t-col>
      </t-row>

      <t-dialog v-model:visible="dialogVisible" attach="body" :header="d.id ? '修改' : '新增'" :footer="null">
        <template #body>
          <t-form ref="form" :data="d" reset-type="initial" :rules="rules" @submit="onSubmit">
            <t-form-item label="名称" name="name">
              <t-input v-model="d.name" />
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
import { ref, onMounted, nextTick } from 'vue';
import { MessagePlugin } from 'tdesign-vue-next';
import request from '@/utils/request';

// 列表数据
const columns: any = [
  { colKey: 'id', title: 'id', width: 64, ellipsis: true, fixed: 'left' },
  { colKey: 'name', title: '名称', width: 100, ellipsis: true, fixed: 'left' },
  { colKey: 'created_at', title: '创建时间', width: 200, ellipsis: true },
  { colKey: 'updated_at', title: '修改时间', width: 200, ellipsis: true },
  { colKey: 'op', title: '操作', align: 'center', fixed: 'right', width: 160 },
];
// 分页数据
const pagination = ref({
  current: 1,
  pageSize: 10,
  total: 0,
});
const table: any = ref({});
const form: any = ref({});
const dList = ref([]);
const selectRow: any = ref({});
const treeData = ref([]);
const menus = ref([]);
const allChildMenuIds = ref([]);
const d: any = ref({});
const dialogVisible = ref(false);
const isLoading = ref(false);
const tree = ref(null);

const list: any = async (page = pagination.value) => {
  isLoading.value = true;
  try {
    const res: any = await request.get('/api/role', {
      params: { page: page.current, limit: page.pageSize },
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
// 只保留包含在全部子节点id中的id
const uniqueChildMenuIds = (ids) => {
  let uniqueChild = [];
  allChildMenuIds.value.forEach((e) => {
    ids.forEach((i) => {
      if (e === i) {
        uniqueChild.push(i);
      }
    });
  });
  return uniqueChild;
};

// 获取全部子节点id
const getAllChildMenuIds = (menus) => {
  menus.forEach((e) => {
    if (e.children && e.children.length > 0) {
      getAllChildMenuIds(e.children);
    } else {
      allChildMenuIds.value.push(e.id);
    }
  });
};
// 转换菜单数据
const menuData = (menus) => {
  if (menus) {
    return menus.map((item) => {
      const m = { label: item.name, value: item.id, children: null };
      if (item.children && item.children.length > 0) {
        m.children = menuData(item.children);
      }
      return m;
    });
  }
  return [];
};
// 获取菜单树
const menuList = async () => {
  const res: any = await request.get('/api/menu');
  if (res.data) {
    menus.value = menuData(res.data);
    // console.log('menus:', JSON.stringify(menus.value));
    getAllChildMenuIds(res.data);
    console.log('allChildMenuIds:', JSON.stringify(allChildMenuIds.value));
  }
};

// 初始数据
onMounted(async () => {
  await list();
  await menuList();
});
// 选择行
const rowClick = ({ row }) => {
  console.log('select row:', row);
  treeData.value = [];
  // 移除父节点
  if (row.menu_ids && row.menu_ids.length > 0) {
    treeData.value = uniqueChildMenuIds(row.menu_ids);
    console.log('treeData:', JSON.stringify(treeData.value));
  }
  selectRow.value = row;
};
// 树选择
const treeChange = async (val) => {
  console.log('treeChange:', JSON.stringify(val));
  await nextTick();
  const allPids = tree.value
    .getItems()
    .filter((e) => e.indeterminate)
    .map((e) => e.value);
  console.log('allPids:', JSON.stringify(allPids));
  selectRow.value.menu_ids = val.concat(allPids);
};
// 新增
const add = () => {
  dialogVisible.value = true;
  form.value.reset();
  form.value.clearValidate();
};
// 修改
const update = (row) => {
  dialogVisible.value = true;
  d.value = Object.assign(d.value, row);
};
// 表单验证
const rules: any = {
  name: [{ required: true, type: 'error' }],
};
// 保存修改数据
const saveOrUpdate = async (params) => {
  const res: any = await request.post('/api/role', params);
  if (res.code === 0) {
    MessagePlugin.success('处理成功');
    dialogVisible.value = false;
    await list();
  }
};
// 保存菜单
const saveMenu = () => {
  if (!selectRow.value.id) {
    MessagePlugin.warning('请先选择角色');
    return;
  }
  saveOrUpdate({
    ...selectRow.value,
    menu_ids: selectRow.value.menu_ids.map(Number),
  });
};
// 提交
const onSubmit = async ({ validateResult, firstError, e }) => {
  e.preventDefault();
  if (validateResult === true) {
    saveOrUpdate(d.value);
  } else {
    console.log(firstError);
  }
};
// 删除数据
const del = async (ids) => {
  const res: any = await request.delete('/api/role', { data: ids });
  if (res.code === 0) {
    MessagePlugin.success('删除成功');
    await list();
  }
};
</script>

<style lang="less" scoped></style>
