import router from "@/routers/router";
import NProgress from "@/config/nprogress";

/**
 * @description 路由拦截 beforeEach
 * */
router.beforeEach(async (to, from, next) => {
    NProgress.start();
    // * 在跳转路由之前，清除所有的请求
    next()
    NProgress.done();
})

router.afterEach(() => {
    NProgress.done();
});

export default router;
