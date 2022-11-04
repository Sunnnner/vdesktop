import { createApp } from "vue";
import App from "./App.vue";
import naive from 'naive-ui'
// 通用字体
import 'vfonts/Lato.css'
// 等宽字体
import 'vfonts/FiraCode.css'
// vue Router
import router from "@/routers/index";
import pinia from "@/store/index";

const app = createApp(App)

app.use(naive).use(router).use(pinia).mount("#app");
