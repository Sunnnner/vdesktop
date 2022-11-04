<template>
    <dark-mode-container class="global-header flex-y-center h-full">
        <global-logo :show-title="true" class="h-full" />
        <div class="flex-1-hidden flex-y-center h-full">
            <menu-collapse />
            <global-breadcrumb />
        </div>
        <header-menu />
        <div class="flex justify-end h-full">
            <github-site />
            <theme-mode />
            <system-message />
            <setting-button v-if="showButton" />
            <user-avatar />
        </div>
    </dark-mode-container>
</template>
  
<script setup lang="ts">

import {
    FullScreen,
    GithubSite,
    GlobalBreadcrumb,
    HeaderMenu,
    MenuCollapse,
    SettingButton,
    SystemMessage,
    ThemeMode,
    UserAvatar
} from './components';

const showButton = import.meta.env.PROD && import.meta.env.VITE_VERCEL !== 'Y';
</script>
  
<style scoped>
.global-header {
    box-shadow: 0 1px 2px rgb(0 21 41 / 8%);
}
</style>
  