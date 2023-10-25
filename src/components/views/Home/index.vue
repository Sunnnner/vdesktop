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
  <n-radio-group name="radiogroup" v-model:value="value" :click="testClick(value)" :default-value="defaultValue">
    <n-space>
      <n-radio v-for="song in songs" :key="song.value" :value="song.value" >
        {{ song.label }}
      </n-radio>
    </n-space>
  </n-radio-group>
  </n-card>
  <n-card :bordered="false" style="display: contents; text-align: center;">
    <n-gradient-text
    gradient="linear-gradient(200deg, red 0%, green 50%, blue 100%)" size="large"
    >
    当前服务器:    {{server_value}}
    </n-gradient-text>
    
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
const value = ref(null)
const defaultValue = ref("")
const server_value = ref("")


const songs = [
        {
          value: "beijing",
          label: "北京服务器"
        },
        {
          value: 'tianjing',
          label: '天津服务器'
        }
      ]

const testClick = (value: any) => {
  if (value === "beijing"){
    invoke("save_server_yaml_file", {server: "北京"})
    invoke("switch_bj_server")
    window.location.reload()
  }else if (value === "tianjing"){
    invoke("save_server_yaml_file", {server: "天津"})
    invoke("switch_tj_server")
    window.location.reload()  
  }
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

invoke('read_server_yaml_file').then((res: any) => {
  server_value.value = res
}).catch((e: any) => {
  message.error(e.message)
})


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