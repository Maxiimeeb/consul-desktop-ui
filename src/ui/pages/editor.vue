<script setup lang="ts">
import VRoundedButton from '../components/v-round-button.vue';
import VButton from '../components/VButton.vue';
import "@fontsource/poppins"; // Defaults to weight 400
import {VueMonacoDiffEditor, VueMonacoEditor} from '@guolao/vue-monaco-editor'
import {computed, onMounted, ref, shallowRef} from "vue";
import yaml from 'yaml';
import {ConsulClient, TauriBride} from "../../core/tauri-bride.ts";
import {toast} from "vue3-toastify";
import {ChangePageFn} from "./use-page-manager.ts";
import {AnOutlinedDisconnect, GlCancel, AnFilledSave, CaCompare, CoFilter} from '@kalimahapps/vue-icons';

enum EditorMode {
  Edit,
  Compare,
}

const props = defineProps<{
  consulClient: ConsulClient,
  changePage: ChangePageFn,
}>()

const bridge = new TauriBride();

const sections = ref<string[]>([]);
const filteredSection = ref<string | null>(null);

const initialParsedYaml = ref<any>({});
const currentParsedYaml = ref<any>({});

const currentValues = ref('');
const mode = ref(EditorMode.Edit);
const editorRef = shallowRef()
const showSections = ref<boolean>(false);
const handleMount = (editor: unknown) => (editorRef.value = editor)

const compareInitial = computed(() => {
  return yaml.stringify(initialParsedYaml.value);
})
const compareCurrent = computed(() => {
  return yaml.stringify(currentParsedYaml.value);
})

const options = ref({
  automaticLayout: true,
  formatOnType: true,
  formatOnPaste: true,
  readOnly: false,
});

/**
 * Call Tauri API to save the cu
 */
const handleSave = async () => {
  applyEditorChanges();

  const result = await toast.promise(bridge.saveConsulValues(
      props.consulClient,
      initialParsedYaml.value,
      currentParsedYaml.value
  ), {
    pending: 'Saving values...',
    success: 'Values saved',
    error: {
      render(err) {
        return `Failed to save values: ${err.data}`;
      }
    },
  });
  initialParsedYaml.value = result;
  
  // Reset everything to the initial state
  handleCancel();
}

/**
 * Cancel the changes and return to the initial values
 */
const handleCancel = () => {
  currentParsedYaml.value = initialParsedYaml.value;

  currentValues.value = filteredSection.value
    ? yaml.stringify({
        [filteredSection.value]: (initialParsedYaml.value)[filteredSection.value],
      })
    : yaml.stringify(initialParsedYaml.value);
}

/**
 * Cancel the changes and return to the initial values
 */
const handleCompare = () => {
  applyEditorChanges();
  
  mode.value = mode.value === EditorMode.Edit ? EditorMode.Compare : EditorMode.Edit
}

/**
 * Define if the user can save or cancel the changes. User can save
 * or cancel if the current values are different from the initial values
 */
const canSaveOrCancel = computed(() => {
  return true;
})

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
  applyEditorChanges();
  sections.value = Object.keys(currentParsedYaml.value);
  options.value = {
    ...options.value,
    readOnly: !options.value.readOnly,
  }
  showSections.value = !showSections.value;
}

const handleSectionClick = (section: string) => {
  // User deselected the current section
  if (filteredSection.value === section) {
    applyEditorChanges();
    filteredSection.value = null;
    currentValues.value = yaml.stringify(currentParsedYaml.value);
  } else if (filteredSection.value !== null) { // User switched section
    applyEditorChanges();
    currentValues.value = yaml.stringify({
      [section]: (currentParsedYaml.value)[section],
    });
    filteredSection.value = section;
  } else { // User select a section and had no section selected before
    applyEditorChanges();
    currentValues.value = yaml.stringify({
      [section]: (currentParsedYaml.value)[section],
    });
    filteredSection.value = section;
  }

  // We simulate a click on the filter button to switch back to edit mode
  handleFilter();
}

/**
 * Apply the current editor to the current parsed value.
 * 
 * @throws if the current editor value is not valid YAML
 */
const applyEditorChanges = () => {
  try {
    if (!filteredSection.value) {
      const parsedYaml = yaml.parse(currentValues.value);
      currentParsedYaml.value = parsedYaml;
    } else {
      const parsedYaml = yaml.parse(currentValues.value);
      currentParsedYaml.value[filteredSection.value] = parsedYaml[filteredSection.value];
    }
  } catch(e) {
    toast.error('Editor contains invalid YAML.');
    throw e;
  }
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
    currentValues.value = yaml.stringify(rawValue);
  } catch(e) {
    toast.error(`Unable to connect to the server ${props.consulClient.scheme.toLowerCase()}://${props.consulClient.host}:${props.consulClient.port} - ${e}`);
    handleDisconnect();
  }

})
</script>

<template>
  <div class="absolute top-5 right-35 z-10 flex gap-3">
    <v-rounded-button class="bg-teal-700  font-bold text-4xl w-15 h-15 text-center" :disabled="false" @click="handleFilter">
      <co-filter />
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
  <vue-monaco-editor
      v-if="mode === EditorMode.Edit"
      v-model:value="currentValues"
      theme="vs-dark"
      :options="options"
      @mount="handleMount"
      language="yaml"
  />
  <vue-monaco-diff-editor
      v-else
      :original="compareInitial"
      :modified="compareCurrent"
      theme="vs-dark"
      :options="{
        readOnly: true,
      }"
      language="yaml"
  />
  <div class="absolute bottom-4 right-5 flex gap-3">
    <v-rounded-button class="bg-teal-700  font-bold text-4xl w-15 h-15 text-center" :disabled="false" @click="handleCompare">
      <ca-compare />
    </v-rounded-button>
    <v-rounded-button class="bg-emerald-600 font-bold text-4xl w-15 h-15 text-center" :disabled="!canSaveOrCancel" @click="handleSave">
      <an-filled-save />
    </v-rounded-button>
    <v-rounded-button class="bg-amber-600 font-bold text-4xl w-15 h-15 text-center" :disabled="!canSaveOrCancel" @click="handleCancel">
      <gl-cancel />
    </v-rounded-button>
    <v-rounded-button class="bg-red-800 font-bold text-4xl w-15 h-15 text-center" :disabled="false" @click="handleDisconnect"><an-outlined-disconnect /></v-rounded-button>
  </div>
</template>
