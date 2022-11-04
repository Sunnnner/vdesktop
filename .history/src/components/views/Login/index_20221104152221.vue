<template>
    <n-card >
        <n-row>
            <n-col :span="24">编辑配置信息</n-col>
            <n-col :span="24">
                <n-form ref="formRef" :model="model" :rules="rules">
                    <n-form-item path="age" label="APPID">
                        <n-input v-model:value="model.appid" @keydown.enter.prevent />
                    </n-form-item>
                    <n-form-item path="password" label="SECRET">
                        <n-input v-model:value="model.app_secret" type="password" @keydown.enter.prevent />
                    </n-form-item>
                    <n-form-item path="password" label="域名">
                        <n-input v-model:value="model.host" type="password" @keydown.enter.prevent />
                    </n-form-item>
                    <n-form-item path="password" label="默认VM">
                        <n-input v-model:value="model.default_vm" type="password" @keydown.enter.prevent />
                    </n-form-item>
                    <n-row :gutter="[0, 24]">
                        <n-col :span="24">
                            <div style="display: flex; justify-content: flex-end">
                                <n-button
                                    :disabled="model.app_secret === null && model.default_vm === null && model.appid === null"
                                    round type="primary">
                                    保存配置
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
    FormItemInst,
    FormItemRule,
    useMessage,
    FormRules
} from 'naive-ui'

interface ModelType {
    appid: string | null
    app_secret: string | null
    host: string | null
    default_vm: string | null
}

const formRef = ref<FormInst | null>(null)
const rPasswordFormItemRef = ref<FormItemInst | null>(null)
const message = useMessage()
const modelRef = ref<ModelType>({
    appid: null,
    app_secret: null,
    host: null,
    default_vm: null
})
const model = modelRef.value


const rules: FormRules = {

}


</script>