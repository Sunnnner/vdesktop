import { RouteRecordRaw } from "vue-router";

// 首页模块
const homeRouter: Array<RouteRecordRaw> = [
    {
        path: '/home',
        redirect: "/home/index",
        children: [
            {
                path: "/home/index",
                name: 'home',
                component: () => import('@/components/views/Home/index.vue'),
                meta: {
                    keepAlive: true,
                    // requiresAuth: true,
                    title: "首页",
                    key: "home"
                }
            }
        ]

    }
];

export default productRouter;
