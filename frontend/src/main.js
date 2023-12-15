import { createApp } from 'vue'
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';
import App from './App.vue'
import store from './store'

import { Buffer } from "buffer"; 
global.Buffer = Buffer;

createApp(App).use(store).use(Antd).mount('#app')