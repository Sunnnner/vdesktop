<template>
    <n-card>
      <h3>编辑配置信息：</h3>
  
      <n-form ref="formRef" :model="model" :rules="rules">
        <n-form-item label="APPID" required>
          <n-input v-model:value="model.appid" placeholder="输入appid" />
        </n-form-item>
  
        <n-form-item label="SECRET" required>
          <n-input v-model:value="model.app_secret" placeholder="输入app secret" />
        </n-form-item>
  
        <n-form-item label="域名" required>
          <n-input v-model:value="model.host" placeholder="输入host" />
        </n-form-item>
  
        <n-form-item label="默认VM" required>
          <n-input v-model:value="model.default_vm" placeholder="输入默认启动的虚拟主机" />
        </n-form-item>
  
        <n-row>
          <n-col span="24">
            <div style="display: flex; justify-content: flex-end;">
              <n-button @click="openFile" round type="primary">导入配置文件</n-button>
              <n-button
                @click="saveConfig"
                round
                type="primary"
              >保存</n-button>
            </div>
          </n-col>
        </n-row>
      </n-form>
    </n-card>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { FormInst, FormRules } from 'naive-ui';
  import { invoke } from '@tauri-apps/api/tauri';
  import { open } from '@tauri-apps/api/dialog';
  import { useRouter } from 'vue-router';
  import { useMessage } from 'naive-ui';
  
  interface Model {
    appid: string | null;
    app_secret: string | null;
    host: string | null;
    default_vm: string | null;
  }
  
  const rules: FormRules = {
    appid: [{ required: true, message: '请输入APPID', trigger: 'blur' }],
    app_secret: [{ required: true, message: '请输入SECRET', trigger: 'blur' }],
    host: [{ required: true, message: '请输入域名', trigger: 'blur' }],
    default_vm: [{ required: true, message: '请输入默认启动的虚拟主机', trigger: 'blur' }],
  };
  
  const model = ref<Model>({
    appid: null,
    app_secret: null,
    host: 'https://vdesk.knd.io',
    default_vm: null,
  });
  
  const formRef = ref<FormInst | null>(null);
  const router = useRouter();
  const message = useMessage();
  
  const openFile = async () => {
    const selected = await open({
      filters: [{ name: 'vdesk', extensions: ['yaml', 'yml'] }],
    });
    if (selected) {
      try {
        const data: Partial<Model> = await invoke('read_yaml_file', { path: selected });
        Object.assign(model.value, data);
      } catch (e: any) {
        message.error(e.message);
      }
    } else {
      message.warning('未选择文件');
    }
  };
  
  const saveConfig = async () => {
    if (formRef.value?.validate()) {
      try {
        await invoke('save_yaml_file', { config: model.value });
        message.success('保存成功');
        router.push({ name: 'home' });
      } catch (e: any) {
        message.error(e.message);
      }
    }
  };
  
  onMounted(async () => {
    const isFileExist = await invoke('is_exist_config');
    if (isFileExist) {
      router.push({ name: 'home' });
    }
  });
  
  </script>
  