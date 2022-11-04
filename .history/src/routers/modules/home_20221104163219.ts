import { RouteRecordRaw } from "vue-router";

// 首页模块
const productRouter: Array<RouteRecordRaw> = [
    {
        path: '/home',
        redirect: "/home/index",
        children: [
            {
                path: "/select/index",
                name: 'select-info',
                component: () => import('@/components/select/index.vue'),
                meta: {
                    keepAlive: true,
                    // requiresAuth: true,
                    title: "选择企业与产品线",
                    key: "select-info"
                }
            }
        ]

    }
];

export default productRouter;