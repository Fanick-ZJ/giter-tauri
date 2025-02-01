<script setup lang="ts">
import { inject } from 'vue';
import ShrinkBar from './shrink-bar.vue';
import { viewShrink } from '@/types/key';

defineOptions({
  name: 'ExtendPage'
})

const _viewShrink = inject(viewShrink, () => {})

</script>
<template>
<div class="relative h-full">
  <ShrinkBar @shrink="_viewShrink" class="absolute left-[-5px]"/>
  <RouterView/>
</div>
</template>


<style scoped>

</style>