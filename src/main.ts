import { createApp } from "vue";
import App from "./App.vue";
import naive from 'naive-ui'
// 通用字体
import 'vfonts/Lato.css'
// 等宽字体
import 'vfonts/FiraCode.css'
// vue Router
import router from "@/routers/index";

const app = createApp(App)

app.use(naive).use(router).mount("#app");
