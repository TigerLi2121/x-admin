/* eslint-disable simple-import-sort/imports */
import TDesign from "tdesign-vue-next";
import { createApp } from "vue";

import App from "./App.vue";
import router from "./router";
import { store } from "./store";
import hasPermission from "@/utils/permission";

import "tdesign-vue-next/es/style/index.css";
import "@/style/index.less";
import "@/style/theme.css";
import "./permission";

const app = createApp(App);

app.use(TDesign);
app.use(store);
app.use(router);
app.use(hasPermission);

app.mount("#app");
