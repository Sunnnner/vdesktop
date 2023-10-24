<template>
  <n-card :title="titleValue">
    <n-data-table :columns="columns" :data="data" :bordered="false" />
  </n-card>

  <n-drawer v-model:show="active" :default-width="300" :placement="placement" resizable>
    <n-drawer-content title="操作">
      <n-grid cols="2 400:4 600:6" y-gap="10">
        <n-grid-item>
          <n-popover trigger="hover">
            <template #trigger>
              <n-button type="primary" @click="turnOn">开机</n-button>
            </template>
            <span>开启虚拟机</span>
          </n-popover>
        </n-grid-item>
        <n-grid-item>
          <n-popover trigger="hover">
            <template #trigger>
              <n-button type="primary" @click="turnOff">关机</n-button>
            </template>
            <span>关闭虚拟机</span>
          </n-popover>
        </n-grid-item>
        <n-grid-item>
          <n-popover trigger="hover">
            <template #trigger>
              <n-button type="primary" @click="bootScreen">启动界面</n-button>
            </template>
            <span> 启动Remote viewer</span>
          </n-popover>
        </n-grid-item>
        <n-grid-item>
          <n-popover trigger="hover">
            <template #trigger>
              <n-button type="primary" @click="forceOff">强制关机</n-button>
            </template>
            <span> 强制关机</span>
          </n-popover>
        </n-grid-item>
        <n-grid-item>
          <n-popover trigger="hover">
            <template #trigger>
              <n-button type="primary" @click="lockedVm">锁定</n-button>
            </template>
            <span> 锁定</span>
          </n-popover>
        </n-grid-item>
        <n-grid-item>
          <n-popover trigger="hover">
            <template #trigger>
              <n-button type="primary" @click="unlockedVm">解锁</n-button>
            </template>
            <span>解锁</span>
          </n-popover>
        </n-grid-item>
        <n-grid-item>
          <n-popover trigger="hover">
            <template #trigger>
              <n-button type="primary" :disabled=true>文件复制</n-button>
            </template>
            <span> Copy text or file to in:/home/clipboard[.txt]</span>
          </n-popover>
        </n-grid-item>
      </n-grid>
    </n-drawer-content>
  </n-drawer>
  <n-card :bordered="false" style="display: contents; text-align: center;">
    <n-switch :rail-style="railStyle">
    <template #checked>
      天津服务器
    </template>
    <template #unchecked>
      北京服务器
    </template>
  </n-switch>
  </n-card>
</template>
  
<script lang="ts" setup>
import { CSSProperties, h, onMounted, ref } from "vue";
import { invoke } from '@tauri-apps/api/tauri';
import { DataTableColumns, DrawerPlacement, NButton } from "naive-ui";
import { useMessage } from 'naive-ui';

const message = useMessage()
const titleValue = ref<string>();
titleValue.value = `虚拟机列表`.toString();
const active = ref(false)
const name = ref('')

const railStyle = ({
  focused,
  checked
}: {
  focused: boolean
  checked: boolean
}) => {
  const style: CSSProperties = {}
  console.log(focused, checked)
  if (checked) {
    style.background = '#d03050'
    if (focused) {
      style.boxShadow = '0 0 0 6px #d0305040'
      invoke('switch_tj_server').then((res) => {
        message.success("切换天津服务器成功")
        // window.location.reload()
      }).catch((e: any) => {
        message.error(e)
      })
    }
  } else {
    style.background = '#2080f0'
    if (focused) {
      style.boxShadow = '0 0 0 6px #2080f040'
      invoke('switch_bj_server').then((res) => {
        message.success("切换北京服务器成功")
        // window.location.reload()
      }).catch((e: any) => {
        message.error(e)
      })
    }
  }
  return style
}

type Song = {
  no: number
  name: string
  locked: string | null
}

const columns: DataTableColumns<Song> = [
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
    render: (row) => {
      return h(
        NButton,
        {
          strong: true,
          tertiary: true,
          size: 'small',
          type: 'primary',
          onClick: () => play(row)
        },
        { default: () => '操作' }
      )
    }
  }
]

const placement = ref<DrawerPlacement>('right')

const play = (row: Song) => {
  active.value = true
  name.value = row.name
}

const forceOff = () => {
  invoke('force_off_vm', { name: name.value }).then((res) => {
    message.success("强制关机成功")
    window.location.reload()
  }).catch((e: any) => {
    message.error(e.message)
  })
}

const lockedVm = () => {
  invoke('locked_vm', { name: name.value }).then((res) => {
    message.success("锁定成功")
    window.location.reload()
  }).catch((e: any) => {
    message.error(e.message)
  })
}

const unlockedVm = () => {
  invoke('unlocked_vm', { name: name.value }).then((res) => {
    message.success("解锁成功")
    window.location.reload()
  }).catch((e: any) => {
    message.error(e.message)
  })
}

const turnOn = () => {
  invoke('turn_on_vm', { name: name.value }).then((res) => {
    message.success("开机成功")
    window.location.reload()
  }).catch((e: any) => {
    message.error(e.message)
  })
}

const turnOff = () => {
  invoke('turn_off_vm', { name: name.value }).then((res) => {
    message.success("关机成功")
    window.location.reload()
  }).catch((e: any) => {
    message.error(e.message)
  })
}

const bootScreen =()=> {
  invoke('boot_screen', { name: name.value }).then((res) => {
    message.success("启动界面成功")
    window.location.reload()
  }).catch((e: any) => {
    message.error(e.message)
  })
}

const data = ref<Song[]>()

interface User{
  id: number;
  name: string;
}

interface Machine{
  id: Number,
  name: String,
  vmid: String,
  node: Node,
  locked_by: User,
}

function get_list_vm(){
  invoke('list_vm').then((res: any) => {
    data.value = res.map((item: Machine) => {
      return {
        no: item.id,
        name: item.name,
        locked: item.locked_by?item.locked_by.name:null
      }
    })
  }).catch((e: any) => {
    message.error(e.message)
  })
}



onMounted(async () => {
  get_list_vm()

  
  // 定时任务每10s刷新页面
  // setInterval(() => {
  //   window.location.reload()
  // }, 10000)
})

</script>

<style scoped>

</style>