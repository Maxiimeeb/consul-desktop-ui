<script setup lang="ts">
import Editor from './ui/pages/editor.vue';
import ServerList from "./ui/pages/server-list.vue";
import SmartUi from "./ui/pages/smart-ui.vue";
import {usePageManager} from "./ui/pages/use-page-manager";

const {
  pageInfo,
  changePage,
} = usePageManager();
</script>

<template>
  <main class="h-12/12">
    <server-list v-if="pageInfo.name === 'server-list'" :change-page="changePage" />
    <editor
        v-else-if="pageInfo.name === 'editor'"
        :consul-client="pageInfo.props.consulClient"
        :change-page="changePage"
    />
    <smart-ui
        v-else-if="pageInfo.name === 'smart-ui'"
        :consul-client="pageInfo.props.consulClient"
        :change-page="changePage"
    />
  </main>
</template>

<style>
@import "tailwindcss";

body {
  font-family: Poppins, sans-serif;
  @apply bg-neutral-800 text-gray-200;
}
</style>
