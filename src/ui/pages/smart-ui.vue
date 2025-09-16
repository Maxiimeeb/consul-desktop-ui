<script setup lang="ts">
import VRoundedButton from '../components/v-round-button.vue';
import VButton from '../components/VButton.vue';
import VConsulEntry from '../components/v-consul-entry.vue';
import "@fontsource/poppins"; // Defaults to weight 400
import {onMounted, ref} from "vue";
import yaml from 'yaml';
import {ConsulClient, TauriBride} from "../../core/tauri-bride.ts";
import {toast} from "vue3-toastify";
import {ChangePageFn} from "./use-page-manager.ts";
import {AnOutlinedDisconnect, CoFilter, FlFilledArrowSwap} from '@kalimahapps/vue-icons';

type ConsulEntry = [string, string];

const props = defineProps<{
  consulClient: ConsulClient,
  changePage: ChangePageFn,
}>()

const bridge = new TauriBride();

const sections = ref<string[]>([]);
const filteredSection = ref<string | null>(null);

const initialParsedYaml = ref<any>({});
const currentParsedYaml = ref<any>({});

const currentValues = ref<ConsulEntry[]>([]);
const showSections = ref<boolean>(false);

const flattenObject = (obj: object, prefix = "") => {
  const result: ConsulEntry[] = [];

  for (const [key, value] of Object.entries(obj)) {
    const newKey = prefix ? `${prefix}/${key}` : key;

    if (typeof value === "string") {
      result.push([newKey, value]);
    } else {
      result.push(...flattenObject(value, newKey));
    }
  }

  return result;
}

/**
 * Returns to the server list page
 */
const handleDisconnect = () => {
  props.changePage({
    name: 'server-list',
    props: {},
  });
}

/**
 * If the file is valid YAML, extract the sections and show them
 */
const handleFilter = () => {
  // If we are already showing the sections, we reset the editor to show all
  sections.value = Object.keys(currentParsedYaml.value);
  showSections.value = !showSections.value;
}

const handleSectionClick = (section: string) => {
  // User deselected the current section
  if (filteredSection.value === section) {
    filteredSection.value = null;
    currentValues.value = currentParsedYaml.value;
  } else if (filteredSection.value !== null) { // User switched section
    currentValues.value = flattenObject({
      [section]: (currentParsedYaml.value)[section],
    });
    filteredSection.value = section;
  } else { // User select a section and had no section selected before
    currentValues.value = flattenObject({
      [section]: (currentParsedYaml.value)[section],
    });
    filteredSection.value = section;
  }

  // We simulate a click on the filter button to switch back to edit mode
  handleFilter();
}

/**
 * Load the base value form the server
 */
onMounted(async () => {
  try {
    const rawValue = await bridge.getConsulValues(props.consulClient);
    initialParsedYaml.value = rawValue;
    currentParsedYaml.value = rawValue;
    sections.value = Object.keys(rawValue);
    currentValues.value = flattenObject(rawValue);
  } catch(e) {
    toast.error(`Unable to connect to the server ${props.consulClient.scheme.toLowerCase()}://${props.consulClient.host}:${props.consulClient.port} - ${e}`);
    handleDisconnect();
  }

})
</script>

<template>
  <div class="absolute top-5 right-5 z-10 flex gap-3">
    <v-rounded-button class="bg-teal-700  font-bold text-4xl w-15 h-15 text-center" :disabled="false" @click="handleFilter">
      <co-filter />
    </v-rounded-button>
    <v-rounded-button class="bg-cyan-700  font-bold text-4xl w-15 h-15 text-center" :disabled="false" @click="() => {
      changePage({
        name: 'editor',
        props: {
          consulClient: props.consulClient,
        },
      })
    }">
      <fl-filled-arrow-swap />
    </v-rounded-button>
  </div>
  <div v-if="showSections" class="absolute top-5 left-5 z-10 flex flex-wrap gap-3 text-white font-poppins w-3/4 overflow-hidden p-1">
    <div v-for="value in sections" :key="value">
      <v-button 
        :class="filteredSection === value ? 'bg-amber-900 rounded-md' : 'bg-amber-700 rounded-md'"
        :disabled="false"
        @click="handleSectionClick(value)"
      >{{ value }}</v-button>
    </div>
  </div>
  <div>
    <div v-for="[key, value] in currentValues" :key="key" class="flex flex-col m-5">
      <v-consul-entry :consul-key="key" :raw-value="value" @update:raw-value="(newValue) => {
        toast.info(`Changed ${key} to ${newValue}`);
      }" />
    </div>
  </div>
  <div class="absolute bottom-4 right-5 flex gap-3">
    <v-rounded-button class="bg-red-800 font-bold text-4xl w-15 h-15 text-center" :disabled="false" @click="handleDisconnect"><an-outlined-disconnect /></v-rounded-button>
  </div>
</template>
