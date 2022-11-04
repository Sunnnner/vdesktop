<template>
 <n-card title="卡片">
    卡片内容
  </n-card>
  </template>
  
<script lang="ts" setup>
import {GlobalStore} from "@/store";
import { ref } from "vue";

const globalStore = GlobalStore();
const title = ref<String>();
</script>

  <style scoped>

  </style>