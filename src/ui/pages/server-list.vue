<script setup lang="ts">
import {ChangePageFn} from "./use-page-manager.ts";
import {onMounted, ref} from "vue";
import VButton from "../components/VButton.vue";
import {CaBareMetalServer, FaFileCirclePlus, FlFilledPlugDisconnected, ChRefresh} from '@kalimahapps/vue-icons';
import {ConsulClient, TauriBride} from "../../core/tauri-bride.ts";
import {toast} from "vue3-toastify";
import VRoundButton from "../components/v-round-button.vue";

const props = defineProps<{
  changePage: ChangePageFn,
}>()

const tauriBride = new TauriBride();
const serverList = ref<(ConsulClient)[]>([]);

const handleConnect = (server: ConsulClient) => {
  props.changePage({
    name: 'editor',
    props: {
      consulClient: server,
    }
  });
}

const loadServers = async () => {
  try {
    serverList.value = await tauriBride.listServers();
  } catch(e) {
    toast.error(`Failed to list servers: ${e}`);
  }
}

onMounted(async () => {
  await loadServers();
});
</script>

<template>
  <div class="h-12/12 flex flex-col items-center justify-center">
    <h1 class="text-7xl font-bold mb-4"><ca-bare-metal-server /></h1>
    <ul class="space-y-2 rounded-xl">
      <li v-for="server in serverList" :key="server.name" class="p-4 flex flex-row items-center gap-4">
        <div class="flex flex-col gap-1">
          <h2 class="text-xl font-bold">{{ server.name }}</h2>
            <span class="text-xs font-bold text-gray-400 lowercase">[{{ server.scheme }}://{{ server.host }}:{{ server.port }}]</span>
        </div>
        <v-button class="font-bold text-3xl" @click="() => handleConnect(server)" :disabled="false">
          <fl-filled-plug-disconnected />
        </v-button>
      </li>
    </ul>
    <div class="flex gap-4 mt-4">
      <v-round-button :disabled="true">
        <fa-file-circle-plus class="text-4xl mt-4" />
      </v-round-button>
      <v-round-button @click="loadServers" :disabled="false" >
        <ch-refresh class="text-4xl mt-4 hover:rotate-90 duration-200" />
      </v-round-button>
    </div>

  </div>

</template>