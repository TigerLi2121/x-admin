export default [
  {
    url: '/api/role',
    method: 'get',
    response: ({ body }) => {
      console.log(body);
      return {
        code: 0,
        msg: 'success',
        total: 10,
        data: [
          {
            id: 1,
            name: '角色1',
            menu_ids: [1, 2, 3],
            created_at: '2020-08-08 08:08:08',
            updated_at: '2020-08-08 08:08:08',
          },
          {
            id: 2,
            name: '角色2',
            menu_ids: [6],
            created_at: '2020-08-08 08:08:08',
            updated_at: '2020-08-08 08:08:08',
          },
          {
            id: 3,
            name: '角色3',
            menu_ids: [1, 9],
            created_at: '2020-08-08 08:08:08',
            updated_at: '2020-08-08 08:08:08',
          },
        ],
      };
    },
  },
  {
    url: '/api/role',
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
    url: '/api/role',
    method: 'delete',
    response: ({ body }) => {
      console.log(body);
      return {
        code: 0,
        msg: 'success',
      };
    },
  },
];
