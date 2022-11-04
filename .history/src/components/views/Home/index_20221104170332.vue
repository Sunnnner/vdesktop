<template>
    <n-space vertical size="large">
      <n-layout>
        <n-layout-header>
            <n-layout-header-title>Header</n-layout-header-title>
        </n-layout-header>
        <n-layout has-sider>
          <n-layout-sider content-style="padding: 24px;">
            <n-layout-sider
                bordered
                collapse-mode="width"
                :collapsed-width="64"
                :width="240"
                :collapsed="collapsed"
                show-trigger
                @collapse="collapsed = true"
                @expand="collapsed = false"
            >
                <n-menu
                :collapsed="collapsed"
                :collapsed-width="64"
                :collapsed-icon-size="22"
                :options="menuOptions"
                :render-label="renderMenuLabel"
                :render-icon="renderMenuIcon"
                :expand-icon="expandIcon"
                />
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
  .n-layout-header,
  .n-layout-footer {
    background: rgba(128, 128, 128, 0.2);
    padding: 24px;
  }
  
  .n-layout-sider {
    background: rgba(128, 128, 128, 0.3);
  }
  
  .n-layout-content {
    background: rgba(128, 128, 128, 0.4);
  }
  </style>