<template>
  <hover-container class="w-40px h-full" tooltip-content="全屏" :inverted="theme.header.inverted" @click="toggle">
    <icon-gridicons-fullscreen-exit class="text-18px" />
    <icon-gridicons-fullscreen v-else class="text-18px" />
  </hover-container>
</template>

<script lang="ts" setup>
</script>

<style scoped></style>
