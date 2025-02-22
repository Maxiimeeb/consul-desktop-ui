<script setup lang="ts">
import VButton from './VButton.vue';
import "@fontsource/poppins"; // Defaults to weight 400
import { VueMonacoEditor, VueMonacoDiffEditor } from '@guolao/vue-monaco-editor'
import {computed, onMounted, ref, shallowRef} from "vue";

enum EditorMode {
  Edit,
  Compare,
}

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
 * TODO
 */
const handleSave = () => {
  console.log('save');
}

/**
 * TODO
 */
const canSaveOrCancel = computed(() => {
  return initialValues.value !== currentValues.value;
})

/**
 * TODO
 */
const editorModeText = computed(() => {
  return mode.value === EditorMode.Edit ? 'COMPARE' : 'EDIT';
})

/**
 * Load the base value form the server
 */
onMounted(() => {
  initialValues.value = 'ALLO: 3\nTest:\n\tVAL: abd # Wow';
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
      <v-button class="bg-blue-500 font-bold w-30" :text="editorModeText" :disabled="false" @click="mode = mode === EditorMode.Edit ? EditorMode.Compare : EditorMode.Edit" />
      <v-button class="bg-green-600 font-bold" text="SAVE" :disabled="!canSaveOrCancel" @click="handleSave"/>
      <v-button class="bg-red-400 font-bold" text="CANCEL" :disabled="!canSaveOrCancel" @click="handleSave"/>
    </div>
</template>