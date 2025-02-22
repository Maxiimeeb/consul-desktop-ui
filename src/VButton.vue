<script setup lang="ts">
defineProps<{
  text: string,
  disabled: boolean,
}>()

const emit = defineEmits<{
  click: [],
}>()
</script>

<template>
  <button
      @click="() => emit('click')"
      class="py-2 px-4 rounded-xl transition duration-100"
      :class="{
        'cursor-not-allowed': disabled,
        'cursor-pointer': !disabled,
        'hover:brightness-90': !disabled,
        'brightness-70': disabled,
      }"
  >
    {{ text }}
  </button>
</template>

<style scoped>

</style>