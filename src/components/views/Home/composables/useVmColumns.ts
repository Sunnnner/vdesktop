import { h } from 'vue'
import { NButton } from 'naive-ui'
import type { DataTableColumns } from 'naive-ui'
import { VmTableData } from '@/types/vm'


export function createVmColumns(onOperation: (row: VmTableData) => void): DataTableColumns<VmTableData> {
  return [
    {
      title: '序号',
      key: 'no',
      width: 100,
      align: 'center'
    },
    {
      title: '虚拟机名称',
      key: 'name',
      width: 200,
      align: 'center'
    },
    {
      title: '是否锁定',
      key: 'locked',
      width: 200,
      align: 'center'
    },
    {
      title: '操作',
      key: 'action',
      width: 200,
      align: 'center',
      render: (row) => h(
        NButton,
        {
          strong: true,
          tertiary: true,
          size: 'small',
          type: 'primary',
          onClick: () => onOperation(row)
        },
        { default: () => '操作' }
      )
    }
  ]
} 