<template>
    <n-card>
      <h3>编辑配置信息：</h3>
  
      <n-form ref="formRef" :model="model" :rules="rules">
        <n-form-item label="APPID" required>
          <n-input v-model:value="model.appid" placeholder="输入appid" />
        </n-form-item>
  
        <n-form-item label="SECRET" required>
          <n-input v-model:value="model.appsecret" placeholder="输入app secret" />
        </n-form-item>
        
        <n-form-item label="姓名" required>
          <n-input v-model:value="model.name" placeholder="中文拼音" />
        </n-form-item>
        <n-space>
          <n-form-item label="指定区域" required> </n-form-item>
            <n-radio
              :checked="checkedValue === 'beijing'"
              value="beijing"
              @change="handleChange"
            >
            北京
            </n-radio>
            <n-radio
              :checked="checkedValue === 'tianjing'"
              value="tianjing"
              @change="handleChange"
            >
              天津
            </n-radio>
        </n-space>

        <n-row>
          <n-col span="24">
            <div style="display: flex; justify-content: flex-end;">
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
  import { invoke } from "@tauri-apps/api/core"
  import { useRouter } from 'vue-router';
  import { useMessage } from 'naive-ui';
  

  interface Model {
    appid: string | null;
    appsecret: string | null;
    url: string | null;
    name: string | null;
    server: string | null;
  }
  
  const rules: FormRules = {
    appid: [{ required: true, message: '请输入APPID', trigger: 'blur' }],
    appsecret: [{ required: true, message: '请输入SECRET', trigger: 'blur' }],
    url: [{ required: true, message: '请输入域名', trigger: 'blur' }],
    name: [{ required: true, message: '请输入默认启动的虚拟主机', trigger: 'blur' }],
  };
  const checkedValue = ref<string | null>(null)
  
  const model = ref<Model>({
    appid: null,
    appsecret: null,
    url: null,
    name: null,
    server: null,
  });
  
  const formRef = ref<FormInst | null>(null);
  const router = useRouter();
  const message = useMessage();
  
  const updateConfig = (server: string| null) => {
    if (server === 'beijing') {
      model.value.url = 'https://vdesk.knd.io';
      model.value.server = 'beijing';
    } else if (server === 'tianjing') {
      model.value.url = 'https://vdesk-tj.knd.io';
      model.value.server = 'tianjing';
    }
  };

  const saveConfig = async () => {
    if (formRef.value?.validate()) {
      updateConfig(checkedValue.value);
      try {
        await invoke('save_config', { config: model.value });
        message.success('保存成功');
        router.push({ name: 'home' });
      } catch (e: any) {
        message.error(e);
      }
    }
  };

  const handleChange = (e: Event) => {
    checkedValue.value = (e.target as HTMLInputElement).value;
    updateConfig(checkedValue.value);
  };
  onMounted(async () => {
    const isFileExist = await invoke('is_exist_config');
    if (isFileExist) {
      router.push({ name: 'home' });
    }
  });
  
  </script>
  