<template>
    <n-card>
        
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