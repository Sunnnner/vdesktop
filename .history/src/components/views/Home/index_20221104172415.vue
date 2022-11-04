<template>
  <n-card :title="titleValue">
    <n-row>
      <n-col :span="24">
        <h3>操作</h3>
      </n-col>
      <n-col :span="24">
        操作
      </n-col>
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