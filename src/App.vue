<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import { onMounted, Ref, ref } from 'vue';

// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;
type Branch = {
  name: string,
  is_remote: boolean,
  reference: string,
}
const branches: Ref<Branch[]> = ref([])
const appWindow = getCurrentWindow();
const watchRepo = () => {
  invoke('add_watch', {
    path: "E:\\workSpace\\Rust\\GQL"
  }).then(() => {
    console.log("finished")
  })
      .catch(e => {
    console.log(e)
  })
}

const getBranches = () => {
  invoke('branches', {
    repo: "E:\\workSpace\\Rust\\GQL"
  }).then((res: Branch[]) => {
    branches.value = res
    console.log(res)
  })
     .catch(e => {
    console.log(e)
  })
}

const getAuthors = () => {
  invoke('authors', {
    repo: "E:\\workSpace\\Rust\\GQL",
    branch: branches.value[0]
  }).then((res) => {
    console.log(res)
  })
    .catch(e => {
    console.log(e)
  })
}

onMounted(() => {
  listen('emit_test', (event) => {
    console.log(event)
  })
  document
  .getElementById('titlebar-minimize')
  ?.addEventListener('click', () => appWindow.minimize());
document
  .getElementById('titlebar-maximize')
  ?.addEventListener('click', () => appWindow.toggleMaximize());
document
  .getElementById('titlebar-close')
  ?.addEventListener('click', () => appWindow.close());
})
</script>

<template>
  <div>
<!--    <div data-tauri-drag-region class="titlebar">-->
<!--      <div class="titlebar-button" id="titlebar-minimize">-->
<!--        <img-->
<!--            src="https://api.iconify.design/mdi:window-minimize.svg"-->
<!--            alt="minimize"-->
<!--        />-->
<!--      </div>-->
<!--      <div class="titlebar-button" id="titlebar-maximize">-->
<!--        <img-->
<!--            src="https://api.iconify.design/mdi:window-maximize.svg"-->
<!--            alt="maximize"-->
<!--        />-->
<!--      </div>-->
<!--      <div class="titlebar-button" id="titlebar-close">-->
<!--        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />-->
<!--      </div>-->
<!--    </div>-->
    <button @click="watchRepo">add_watch</button>
    <button @click="getBranches">get branches</button>
    <button @click="getAuthors">get authors</button>
  </div>
</template>

<style scoped>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}
.titlebar-button:hover {
  background: #5bbec3;
}

</style>