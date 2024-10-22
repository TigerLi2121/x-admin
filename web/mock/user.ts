const userinfo = {
  id: 1,
  created_at: '2023-07-10 18:32:32',
  updated_at: '2023-07-10 18:32:32',
  username: 'admin',
  password: '',
  email: '888@qq.com',
  mobile: '18888888888',
  status: 1,
  role_ids: null,
  menus: [
    {
      path: '/user',
      name: 'userParent',
      component: 'LAYOUT',
      redirect: '/user/user',
      meta: { title: '用户管理', icon: 'user-1', single: true },
      children: [
        {
          path: 'user',
          name: 'user',
          component: '/user',
          meta: { title: '用户管理', icon: 'user-1' },
          children: [],
        },
      ],
    },
    {
      path: '/menu',
      name: 'menuParent',
      component: 'LAYOUT',
      redirect: '/menu/menu',
      meta: { title: '菜单管理', icon: 'menu', single: true },
      children: [
        {
          path: 'menu',
          name: 'menu',
          component: '/menu',
          meta: { title: '菜单管理', icon: 'menu' },
          children: [],
        },
      ],
    },
    {
      path: '/role',
      name: 'roleParent',
      component: 'LAYOUT',
      redirect: '/role/role',
      meta: { title: '角色管理', icon: 'transform', single: true },
      children: [
        {
          path: 'role',
          name: 'role',
          component: '/role',
          meta: { title: '角色管理', icon: 'transform' },
          children: [],
        },
      ],
    },
  ],
  perms: [
    'role:delete',
    'role:update',
    'role:add',
    'user:delete',
    'user:update',
    'user:add',
    'menu:delete',
    'menu:update',
    'menu:add',
  ],
};

export default [
  {
    url: '/api/login',
    method: 'post',
    response: ({ body }) => {
      console.log(body);
      if (body.username !== body.password) {
        return {
          code: 500,
          msg: '密码错误',
        };
      }
      return {
        code: 0,
        msg: 'success',
        data: {
          token: 'admin',
        },
      };
    },
  },
  {
    url: '/api/user/current',
    method: 'get',
    response: ({ body }) => {
      console.log(body);
      return {
        code: 0,
        msg: 'success',
        data: userinfo,
      };
    },
  },
  {
    url: '/api/user',
    method: 'get',
    response: ({ query }) => {
      console.log(query);
      return {
        code: 0,
        msg: 'success',
        total: 18,
        data: [
          {
            id: 1,
            username: 'aaa1',
            email: '111@qq.com',
            mobile: '1231231',
            status: 1,
            role_ids: [2],
            created_at: '2021-01-01 01:01:01',
            updated_at: '2021-01-01 01:01:01',
          },
          {
            id: 2,
            username: 'aaa2',
            email: '111@qq.com',
            mobile: '123123',
            status: 0,
            role_ids: [1, 2],
            created_at: '2021-01-01 01:01:01',
            updated_at: '2021-01-01 01:01:01',
          },
          {
            id: 3,
            username: 'aaa3',
            email: '111@qq.com',
            mobile: '123123',
            status: 1,
            role_ids: [1, 2],
            created_at: '2021-01-01 01:01:01',
            updated_at: '2021-01-01 01:01:01',
          },
          {
            id: 4,
            username: 'aaa4',
            email: '111@qq.com',
            mobile: '123123',
            status: 1,
            role_ids: [1, 2],
            created_at: '2021-01-01 01:01:01',
            updated_at: '2021-01-01 01:01:01',
          },
          {
            id: 5,
            username: 'aaa5',
            email: '111@qq.com',
            mobile: '123123',
            status: 1,
            role_ids: [1],
            created_at: '2021-01-01 01:01:01',
            updated_at: '2021-01-01 01:01:01',
          },
        ],
      };
    },
  },
  {
    url: '/api/user',
    method: 'post',
    response: ({ body }) => {
      console.log(body);
      return {
        code: 0,
        msg: 'success',
      };
    },
  },
  {
    url: '/api/user',
    method: 'delete',
    response: ({ body }) => {
      console.log(body);
      if (body[0] === 1) {
        return {
          code: 500,
          msg: '不能删除',
        };
      }
      return {
        code: 0,
        msg: 'success',
      };
    },
  },
];
