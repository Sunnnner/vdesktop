<template>
  <div class="home-container">
    <!-- 虚拟机列表 -->
    <n-card :title="title">
      <n-data-table 
        :columns="columns" 
        :data="tableData" 
        :bordered="false" 
      />
    </n-card>

    <!-- 操作抽屉 -->
    <VmOperationDrawer 
      v-model:show="drawerVisible"
      :vmName="selectedVm"
      @refresh="refreshVmList"
    />

    <!-- 服务器选择 -->
    <n-card :bordered="false" class="server-select">
      <n-radio-group v-model:value="currentServer">
        <n-space>
          <n-radio v-for="server in servers"
            :key="server.value"
            :value="server.value"
            @change="handleServerChange"
          >
            {{ server.label }}
          </n-radio>
        </n-space>
      </n-radio-group>
    </n-card>

    <!-- 当前服务器显示 -->
    <n-card :bordered="false" class="server-display">
      <n-gradient-text
        :gradient="serverGradient"
        size="large"
      >
        当前服务器: {{ currentServer }}
      </n-gradient-text>
    </n-card>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, onUnmounted, defineAsyncComponent } from 'vue'
import { useMessage } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import type { VmTableData, Machine } from '@/types/vm'
import { createVmColumns } from './composables/useVmColumns'

const VmOperationDrawer = defineAsyncComponent(() => 
  import('./components/VmOperationDrawer.vue')
)

const message = useMessage()

// 状态管理
const title = ref('虚拟机列表')
const drawerVisible = ref(false)
const selectedVm = ref('')
const currentServer = ref<string | null>(null)
const tableData = ref<VmTableData[]>([])

// 服务器配置
const servers = [
  { label: '北京', value: 'beijing' },
  { label: '天津', value: 'tianjing' }
]

const serverGradient = 'linear-gradient(200deg, red 0%, green 50%, blue 100%)'

// 表格列配置
const columns = createVmColumns((row: VmTableData) => {
  drawerVisible.value = true
  selectedVm.value = row.name
})

// 获取虚拟机列表
const refreshVmList = async () => {
  try {
    const res = await invoke('get_vms') as Machine[]
    tableData.value = res.map(item => ({
      no: item.id,
      name: item.name,
      locked: item.locked_by?.name || null
    }))
  } catch (e: any) {
    message.error(e)
  }
}

// 获取服务器配置
const getServerConfig = async () => {
  try {
    const res: any = await invoke('get_config')
    currentServer.value = res.server
  } catch (e: any) {
    message.error(e)
  }
}

// 切换服务器
const handleServerChange = async () => {
  try {
    await invoke('switch_server', { server: currentServer.value })
    await getServerConfig()
    await refreshVmList()
  } catch (e: any) {
    message.error(e)
  }
}

// 生命周期
onMounted(() => {
  refreshVmList()
  getServerConfig()
  
  const refreshInterval = setInterval(() => {
    refreshVmList()
    getServerConfig()
  }, 10000)

  onUnmounted(() => {
    clearInterval(refreshInterval)
  })
})
</script>

<style scoped>
.home-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.server-select,
.server-display {
  text-align: center;
}
</style>