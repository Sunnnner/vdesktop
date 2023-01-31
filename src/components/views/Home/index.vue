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
              <n-button type="primary" :disabled=true>文件复制</n-button>
            </template>
            <span> Copy text or file to in:/home/clipboard[.txt]</span>
          </n-popover>
        </n-grid-item>
      </n-grid>
    </n-drawer-content>
  </n-drawer>
</template>
  
<script lang="ts" setup>
import { h, onMounted, ref } from "vue";
import { invoke } from '@tauri-apps/api/tauri';
import { DataTableColumns, DrawerPlacement, NButton } from "naive-ui";
import { useMessage } from 'naive-ui';

const message = useMessage()
const titleValue = ref<string>();
titleValue.value = `虚拟机列表`.toString();
const active = ref(false)

const name = ref('')

interface ResValue {
  vms: string;
  locked: string | null;
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

interface ResCode {
  message: string,
  code: string
}

const placement = ref<DrawerPlacement>('right')

const play = (row: Song) => {
  // 如果是锁定状态，禁用button
  if (row.locked !== null) {
    message.info('当前虚拟机已锁定，无法操作')
  } else {
    active.value = true
    name.value = row.name
  }
}

const turnOn = async () => {
  try{
    const res:ResCode = await invoke('turn_on_vm', { name: name.value })
    if (res.code === "1"){
      message.error(res.message)
    } else {
      message.success(res.message)
    }
  } catch (e: any) {
    message.error(e.message)
  }
}

const turnOff = async () => {
  try{
    const res:ResCode = await invoke('turn_off_vm', { name: name.value })
    if (res.code === "1"){
      message.error(res.message)
    } else {
      message.success(res.message)
    }
  } catch (e: any) {
    message.error(e.message)
  }
}

const bootScreen = async () => {
  await invoke('boot_screen', { name: name.value })
}

const data = ref<Song[]>()

onMounted(async () => {
  const res: ResValue[] = await invoke('list_vm');
  data.value = res.map((item, index) => {
    return {
      no: index + 1,
      name: item.vms,
      locked: item.locked
    }
  })
})

</script>

<style scoped>

</style>