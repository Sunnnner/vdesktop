import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { ComputedRef } from 'vue'

export function useVmOperations(vmName: ComputedRef<string>) {
  const message = useMessage()

  const operations = [
    { key: 'start_vms', label: '开机', tooltip: '开启虚拟机' },
    { key: 'stop_vms', label: '关机', tooltip: '关闭虚拟机' },
    { key: 'spice_viewer', label: '启动界面', tooltip: '启动Remote viewer' },
    { key: 'force_stop_vms', label: '强制关机', tooltip: '强制关机' },
    { key: 'lock_vms', label: '锁定', tooltip: '锁定' },
    { key: 'unlock_vms', label: '解锁', tooltip: '解锁' },
    { 
      key: 'file_copy', 
      label: '文件复制', 
      tooltip: 'Copy text or file to in:/home/clipboard[.txt]',
      disabled: true 
    }
  ]

  const handleVmOperation = async (operation: string) => {
    try {
      // 使用 .value 访问计算属性的值
      await invoke(operation, { name: vmName.value })
      message.success(`${operations.find(op => op.key === operation)?.label}成功`)
    } catch (e: any) {
      message.error(e)
    }
  }

  return {
    operations,
    handleVmOperation
  }
} 