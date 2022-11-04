import router from "@/routers/router";
import NProgress from "@/config/nprogress";
import { GlobalStore } from "@/store";
import { AxiosCanceler } from "@/api/helper/axiosCancel";

const axiosCanceler = new AxiosCanceler();

/**
 * @description 路由拦截 beforeEach
 * */
router.beforeEach(async (to, from, next) => {
    NProgress.start();
    // * 在跳转路由之前，清除所有的请求

    axiosCanceler.removeAllPending();
    if (to.path === '/login || to.path === '/') {
        next()
    } else {
        const globalStore = GlobalStore();
        // * 判断是否有Token
        if (!globalStore.config) {
            next({ path: '/login' })
            NProgress.done();
        } else {
            next()
        }
    }
})

router.afterEach(() => {
    NProgress.done();
});

export default router;
