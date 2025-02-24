<script setup lang="ts">
import VButton from './VButton.vue';
import "@fontsource/poppins"; // Defaults to weight 400
import {VueMonacoDiffEditor, VueMonacoEditor} from '@guolao/vue-monaco-editor'
import {computed, onMounted, ref, shallowRef} from "vue";
import yaml from 'yaml';
import {TauriBride} from "./core/tauri-bride.ts";
import {toast} from "vue3-toastify";

enum EditorMode {
  Edit,
  Compare,
}

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
      {
        host: 'localhost',
        port: 8500,
        scheme: 'HTTP',
      },
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
 * Define if the user can save or cancel the changes. User can save
 * or cancel if the current values are different from the initial values
 */
const canSaveOrCancel = computed(() => {
  return initialValues.value !== currentValues.value;
})

/**
 * Change the editor mode text based on the current mode
 */
const editorModeText = computed(() => {
  return mode.value === EditorMode.Edit ? 'COMPARE' : 'EDIT';
})

const foo = async () => {
  toast.info('Test');
}

/**
 * Load the base value form the server
 */
onMounted(async () => {
  const rawValue = await bridge.getConsulValues({
    host: 'localhost',
    port: 8500,
    scheme: 'HTTP',
  });
  initialValues.value = yaml.stringify(rawValue);
  currentValues.value = initialValues.value;
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
    <v-button class="bg-blue-500 font-bold w-30" :text="editorModeText" :disabled="false"
              @click="mode = mode === EditorMode.Edit ? EditorMode.Compare : EditorMode.Edit"/>
    <v-button class="bg-green-600 font-bold" text="SAVE" :disabled="!canSaveOrCancel" @click="handleSave"/>
    <v-button class="bg-red-400 font-bold" text="CANCEL" :disabled="!canSaveOrCancel" @click="handleSave"/>
    <v-button class="bg-red-400 font-bold" text="TEST" :disabled="false" @click="foo"/>
  </div>
</template>