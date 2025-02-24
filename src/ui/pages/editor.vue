<script setup lang="ts">
import VRoundedButton from '../components/v-round-button.vue';
import "@fontsource/poppins"; // Defaults to weight 400
import {VueMonacoDiffEditor, VueMonacoEditor} from '@guolao/vue-monaco-editor'
import {computed, onMounted, ref, shallowRef} from "vue";
import yaml from 'yaml';
import {ConsulClient, TauriBride} from "../../core/tauri-bride.ts";
import {toast} from "vue3-toastify";
import {ChangePageFn} from "./use-page-manager.ts";
import {AnOutlinedDisconnect, GlCancel, AnFilledSave, CaCompare} from '@kalimahapps/vue-icons';

enum EditorMode {
  Edit,
  Compare,
}

const props = defineProps<{
  consulClient: ConsulClient,
  changePage: ChangePageFn,
}>()

const bridge = new TauriBride();
const initialValues = ref('');
const currentValues = ref('');
const mode = ref(EditorMode.Edit);
const editorRef = shallowRef()
const handleMount = (editor: unknown) => (editorRef.value = editor)


const options = {
  automaticLayout: true,
  formatOnType: true,
  formatOnPaste: true,
}

/**
 * Call Tauri API to save the cu
 */
const handleSave = async () => {
  let initial;
  let current;

  try {
    initial = yaml.parse(initialValues.value);
    current = yaml.parse(currentValues.value);
  } catch(e) {
    toast.error(`Failed to save values: ${e}`);
    return;
  }

  const result = await toast.promise(bridge.saveConsulValues(
      props.consulClient,
      initial,
      current,
  ), {
    pending: 'Saving values...',
    success: 'Values saved',
    error: {
      render(err) {
        return `Failed to save values: ${err.data}`;
      }
    },
  });
  initialValues.value = yaml.stringify(result);
  currentValues.value = initialValues.value;
}

/**
 * Cancel the changes and return to the initial values
 */
const handleCancel = () => {
  currentValues.value = initialValues.value;
}

/**
 * Define if the user can save or cancel the changes. User can save
 * or cancel if the current values are different from the initial values
 */
const canSaveOrCancel = computed(() => {
  return initialValues.value !== currentValues.value;
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
 * Load the base value form the server
 */
onMounted(async () => {
  try {
    const rawValue = await bridge.getConsulValues(props.consulClient);
    initialValues.value = yaml.stringify(rawValue);
    currentValues.value = initialValues.value;
  } catch(e) {
    toast.error(`Unable to connect to the server ${props.consulClient.scheme.toLowerCase()}://${props.consulClient.host}:${props.consulClient.port} - ${e}`);
    handleDisconnect();
  }

})
</script>

<template>
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
      :original="initialValues"
      :modified="currentValues"
      theme="vs-dark"
      :options="{
        readOnly: true,
      }"
      language="yaml"
  />
  <div class="absolute bottom-4 right-5 flex gap-3">
    <v-rounded-button class="bg-teal-700  font-bold text-4xl w-15 h-15 text-center" :disabled="false" @click="mode = mode === EditorMode.Edit ? EditorMode.Compare : EditorMode.Edit">
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
