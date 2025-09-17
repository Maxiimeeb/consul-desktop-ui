<script setup lang="ts">
import {ref, onMounted} from "vue";
import { CaCharacterWholeNumber, CaLetterAa, CiText, CaBoolean } from '@kalimahapps/vue-icons';

defineProps<{
  consulKey: string,
}>()
const rawValue = defineModel<string>('rawValue', {
    required: true,
});
const toggleState = ref<boolean>(false);

type Valuetype = 'string' | 'number' | 'boolean' | 'textarea';

const valueType = ref<Valuetype>('string');

const handleBoolChange = () => {
    toggleState.value = !toggleState.value;
    rawValue.value = toggleState.value ? 'True' : 'False';
}

onMounted(() => {
    const value = rawValue.value.toLowerCase();
    if (value.toLowerCase() === 'true' || value === 'false') {
        valueType.value = 'boolean';
        toggleState.value = value === 'true';
    } else if (!isNaN(Number(value))) {
        valueType.value = 'number';
    } else if (value.includes('\n')) {
        valueType.value = 'textarea';
    } else {
        valueType.value = 'string';
    }
});
</script>

<template>
<div class="p-1 border-b-1 border-neutral-600 flex items-center gap-3">
    <div class="w-full flex h-10 items-center gap-3">
        <div class="text-2xl">
            <component
                :is="{
                    string: CaLetterAa,
                    number: CaCharacterWholeNumber,
                    boolean: CaBoolean,
                    textarea: CiText,
                }[valueType]"
                class="text-gray-400"
            />
        </div>
        <label :for="consulKey" class="text-white font-poppins">{{ consulKey }}</label>
    </div>
    <div class="input-section flex flex-row-reverse ml-auto w-full">
        <input
            v-if="valueType === 'string' || valueType === 'number'"
            :type="valueType === 'string' ? 'text' : 'number'"
            :id="consulKey"
            v-model="rawValue"
            class="bg-neutral-700 text-gray-200 p-2 rounded-md w-1/2 font-mono focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <textarea
            v-else-if="valueType === 'textarea'"
            :id="consulKey"
            v-model="rawValue"
            class="bg-neutral-700 text-gray-200 p-2 rounded-md w-3/4
            h-20 font-mono resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
        ></textarea>
        <label class="inline-flex items-center cursor-pointer" v-else-if="valueType === 'boolean'">
            <input @click="handleBoolChange" type="checkbox" class="sr-only peer" :checked="toggleState"/>
            <div class="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"></div>
        </label>
    </div>
</div>
</template>
