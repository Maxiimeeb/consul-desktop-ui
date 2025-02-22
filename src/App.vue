<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="">
    <div @click="greet" class="">allo</div>
    {{ greetMsg }}
  </main>
</template>

<style>
@import "tailwindcss";

body {
  @apply bg-slate-700 text-gray-200;
}
</style>
