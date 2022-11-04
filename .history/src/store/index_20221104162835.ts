import { defineStore, createPinia } from "pinia";
import { Config, GlobalState } from "./interface";
import piniaPersistConfig from "@/config/piniaPersist";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";

// defineStore 调用后返回一个函数，调用该函数获得 Store 实体
export const GlobalStore = defineStore({
  // id: 必须的，在所有 Store 中唯一
  id: "GlobalState",
  // state: 返回对象的函数
  state: (): GlobalState => ({
    // token
    token: "",
    // userInfo
    config: null,
  }),
  getters: {},
  actions: {
    // setToken
    setToken(token: string) {
      this.token = token;
    },
    // setUserInfo
    setConfig(config: Config) {
      this.config = config;
    },
    clearAll() {
      this.token = "";
      this.config = null;
    }
  },
  persist: piniaPersistConfig("GlobalState")
});

// piniaPersist(持久化)
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

export default pinia;
