<script setup lang="ts">
</script>

<template>
  <div class="glass">
    <slot></slot>
  </div>
</template>

<style scoped>
.glass {
  background: rgba(255, 255, 255, 0.01);
  border: none;
  backdrop-filter: blur(2px);
  border-radius: 5px;
  -webkit-backdrop-filter: blur(2px);
}
</style>