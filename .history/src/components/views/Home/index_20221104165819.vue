<template>
    <div class="home">
        <div class="home__header">
        <div class="home__header__logo">
            <img src="@/assets/logo.png" alt="logo" />
        </div>
        <div class="home__header__title">
            <h1>Virtual Desktop</h1>
            <p>Virtual Desktop is a remote desktop application that allows you to access your desktop from anywhere.</p>
        </div>
        </div>
        <div class="home__content">
        <div class="home__content__left">
            <div class="home__content__left__title">
            <h1>Virtual Desktop</h1>
            <p>Virtual Desktop is a remote desktop application that allows you to access your desktop from anywhere.</p>
            </div>
            <div class="home__content__left__form">
            <n-form ref="formRef" :model="model" :rules="rules" label-placement="top">
                <n-form-item label="App ID" prop="appid">
                <n-input v-model:modelValue="model.appid" />
                </n-form-item>
                <n-form-item label="App Secret" prop="app_secret">
                <n-input v-model:modelValue="model.app_secret" />
                </n-form-item>
                <n-form-item label="Host" prop="host">
                <n-input v-model:modelValue="model.host" />
                </n-form-item>
                <n-form-item label="Default VM" prop="default_vm">
                <n-input v-model:modelValue="model.default_vm" />
                </n-form-item>
                <n-form-item>
                <n-button type="primary" @click="submit">Submit</n-button>
                </n-form-item>
            </n-form>
            </div>
        </div>
        <div class="home__content__right">
            <img src="@/assets/home.png" alt="home" />
        </div>
        </div>
    </div>
</template>