<template>
    <n-space vertical size="large">
      <n-layout>
        <n-layout-header>颐和园路</n-layout-header>
        <n-layout has-sider>
          <n-layout-sider content-style="padding: 24px;">
            <n-menu :options="menuOptions" />
          </n-layout-sider>
          <n-layout-content content-style="padding: 24px;">
            平山道
          </n-layout-content>
        </n-layout>
        <n-layout-footer>成府路</n-layout-footer>
      </n-layout>
    </n-space>
  </template>
  
  <style scoped>
  </style>