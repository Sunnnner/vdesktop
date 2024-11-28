<template>
  <n-drawer 
    v-model:show="visible"
    :default-width="300"
    placement="right"
    resizable
  >
    <n-drawer-content title="操作">
      <n-grid cols="2 400:4 600:6" y-gap="10">
        <n-grid-item v-for="operation in operations" :key="operation.key">
          <n-popover trigger="hover">
            <template #trigger>
              <n-button
                type="primary"
                :disabled="operation.disabled"
                @click="handleOperation(operation.key)"
              >
                {{ operation.label }}
              </n-button>
            </template>
            <span>{{ operation.tooltip }}</span>
          </n-popover>
        </n-grid-item>
      </n-grid>
    </n-drawer-content>
  </n-drawer>
</template>

<script lang="ts" setup>
import { computed, toRef } from 'vue'
import { useVmOperations } from '../composables/useVmOperations';

const props = defineProps<{
  show: boolean
  vmName: string
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
  (e: 'refresh'): void
}>()

const visible = computed({
  get: () => props.show,
  set: (value) => emit('update:show', value)
})


// 修改这部分代码
const { operations, handleVmOperation } = useVmOperations(computed(() => props.vmName))

const handleOperation = async (key: string) => {
  await handleVmOperation(key)
  emit('refresh')
}
</script> 