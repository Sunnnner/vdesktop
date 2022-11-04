<template>
    <n-card >
        <n-row>
            <h3>编辑配置信息：</h3>
            
            <n-col :span="24">
                <n-form ref="formRef" :model="model" :rules="rules">
                    <n-form-item path="age" label="APPID">
                        <n-input v-model:value="model.appid"  placeholder="输入姓名"  />
                    </n-form-item>
                    <n-form-item path="password" label="SECRET">
                        <n-input v-model:value="model.app_secret" type="password"  />
                    </n-form-item>
                    <n-form-item label="域名">
                        <n-input v-model:value="model.host" type="password"  />
                    </n-form-item>
                    <n-form-item label="默认VM">
                        <n-input v-model:value="model.default_vm" type="password"  />
                    </n-form-item>
                    <n-row :gutter="[0, 24]">
                        <n-col :span="18">
                            <div style="display: flex; justify-content: flex-end">
                                <n-button
                                    :disabled="model.app_secret === null && model.default_vm === null && model.appid === null"
                                    round type="primary">
                                    导入配置文件
                                </n-button>
                            </div>
                        </n-col>
                        <n-col :span="6">
                            <div style="display: flex; justify-content: flex-end">
                                <n-button
                                    :disabled="model.app_secret === null && model.default_vm === null && model.appid === null"
                                    round type="primary">
                                    保存
                                </n-button>
                            </div>
                        </n-col>
                    </n-row>
                </n-form>
            </n-col>
        </n-row>
    </n-card>

</template>

<script lang="ts" setup>
import { ref } from 'vue'
import {
    FormInst,
    FormRules
} from 'naive-ui'

interface ModelType {
    appid: string | null
    app_secret: string | null
    host: string | null
    default_vm: string | null
}

const formRef = ref<FormInst | null>(null)
const modelRef = ref<ModelType>({
    appid: null,
    app_secret: null,
    host: "https://vdesk.knd.io",
    default_vm: null
})
const model = modelRef.value


const rules: FormRules = {

}


</script>