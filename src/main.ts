import { createApp } from "vue";
import App from "@/App.vue";
import { setupStore } from "@/store";
import './index.scss'

const app = createApp(App);

setupStore(app);

app.mount("#app");
