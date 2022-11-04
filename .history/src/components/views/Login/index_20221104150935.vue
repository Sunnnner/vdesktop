<template>
    <n-form ref="formRef" :model="model" :rules="rules">
        <n-form-item path="age" label="年龄">
            <n-input v-model:value="model.appid" @keydown.enter.prevent />
        </n-form-item>
        <n-form-item path="password" label="密码">
            <n-input v-model:value="model.app_secret" type="password" @input="handlePasswordInput"
                @keydown.enter.prevent />
        </n-form-item>

        <n-row :gutter="[0, 24]">
            <n-col :span="24">
                <div style="display: flex; justify-content: flex-end">
                    <n-button :disabled="model.age === null" round type="primary" @click="handleValidateButtonClick">
                        验证
                    </n-button>
                </div>
            </n-col>
        </n-row>
    </n-form>
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