import {createApp} from "vue";
import App from "./App.vue";
import {loader} from '@guolao/vue-monaco-editor'
import Vue3Toastify, {type ToastContainerOptions} from 'vue3-toastify';
import 'vue3-toastify/dist/index.css';


loader.config({
    paths: {
        vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.43.0/min/vs',
    },
})

createApp(App).use(
    Vue3Toastify,
    {
        autoClose: 3000,
        theme: 'dark',
        // ...
    } as ToastContainerOptions,
).mount("#app")
