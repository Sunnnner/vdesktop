<template>
  <n-card :title="titleValue">
    <n-row>
      <n-col :span="24">
        <h3>操作</h3>
      </n-col>
      <n-col :span="24">
        <n-button>列出所有虚拟机</n-button>
      </n-col>
      <n-col :span="24">
        <n-button>连接当前虚拟机</n-button>
      </n-col>
      <n-col :span="24">
        <n-button>切换虚拟机</n-button>
      </n-col>
      <n-col :span="24">
        <n-button>关闭当前虚拟机</n-button> 
    </n-row>
  </n-card>
</template>
  
<script lang="ts" setup>
import { GlobalStore } from "@/store";
import { ref } from "vue";

const globalStore = GlobalStore();
const titleValue = ref<string>();
titleValue.value = `当前虚拟机名称: ${globalStore.config ? globalStore.config.default_vm : 'default'}`.toString();
</script>

<style scoped>

</style>