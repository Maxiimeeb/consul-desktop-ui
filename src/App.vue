<script setup lang="ts">
import EditorPage from './EditorPage.vue';

</script>

<template>
  <main class="h-12/12">
    <editor-page />
  </main>
</template>

<style>
@import "tailwindcss";

body {
  font-family: Poppins, sans-serif;
  @apply bg-slate-700 text-gray-200;
}
</style>
