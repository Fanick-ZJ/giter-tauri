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
const watchRepo = (path: string) => {
  invoke('add_watch', {
    path: path
  }).then(() => {
    console.log("finished")
  })
      .catch(e => {
    console.log(e)
  })
}

const getBranches = (path: string) => {
  invoke('branches', {
    repo: path
  }).then((res: Branch[]) => {
    branches.value = res
    console.log(res)
  })
     .catch(e => {
    console.log(e)
  })
}

const getAuthors = ( path: string) => {
  invoke('authors', {
    repo: path,
    branch: branches.value[0]
  }).then((res) => {
    console.log(res)
  })
    .catch(e => {
    console.log(e)
  })
}

const clear_all_cache = () => {
  invoke('clear_all_cache').then((res) => {
    console.log(res)
  })
    .catch(e => {
    console.log(e)
  })
}

const clear_cache = (path: string) => {
  invoke('clear_cache', {
    repo: path
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
    <button @click="watchRepo('E:\\workSpace\\Rust\\GQL')">add_watch</button>
    <button @click="getBranches('E:\\workSpace\\Rust\\GQL')">get branches</button>
    <button @click="getAuthors('E:\\workSpace\\Rust\\GQL')">get authors</button>
    <button @click="clear_cache('E:\\workSpace\\Rust\\GQL\\.git')">clear repo</button>
    <p>
      <button @click="clear_all_cache()"> clear </button>
    </p>
    <button @click="watchRepo('E:\\workSpace\\Python_Project_File\\wizvision3')">add_watch</button>
    <button @click="getBranches('E:\\workSpace\\Python_Project_File\\wizvision3')">get branches</button>
    <button @click="getAuthors('E:\\workSpace\\Python_Project_File\\wizvision3')">get authors</button>
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