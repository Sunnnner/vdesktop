<template>
 <n-card :title="当前虚拟机名称:title">
    卡片内容
  </n-card>
  </template>
  
<script lang="ts" setup>
import {GlobalStore} from "@/store";
import { ref } from "vue";

const globalStore = GlobalStore();
const title = ref<String>();
title.value = globalStore.config?globalStore.config.default_vm: 'default';
</script>

  <style scoped>

  </style>