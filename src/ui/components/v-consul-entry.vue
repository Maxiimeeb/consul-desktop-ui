<script setup lang="ts">
import {ref, onMounted} from "vue";

defineProps<{
  consulKey: string,
}>()
const rawValue = defineModel<string>('rawValue', {
    required: true,
});

type Valuetype = 'string' | 'number' | 'boolean';

const valueType = ref<Valuetype>('string');

onMounted(() => {
    const value = rawValue.value.toLowerCase();
    if (value.toLowerCase() === 'true' || value === 'false') {
        valueType.value = 'boolean';
    } else if (!isNaN(Number(value))) {
        valueType.value = 'number';
    } else {
        valueType.value = 'string';
    }
});
</script>

<template>
<div class="bg-neutral-700 p-1 rounded-md">
    <label :for="consulKey" class="text-white font-poppins">{{ consulKey }}</label>
</div>
</template>
