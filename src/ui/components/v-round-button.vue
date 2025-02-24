<script setup lang="ts">
const props = defineProps<{
  disabled: boolean,
}>()

const emit = defineEmits<{
  click: [],
}>()

const handleClick = () => {
  if (props.disabled) return;
  emit('click');
}
</script>

<template>
  <button
      @click="handleClick"
      class="p-1 rounded-full transition duration-100 flex items-center justify-center"
      :class="{
        'cursor-not-allowed': disabled,
        'cursor-pointer': !disabled,
        'hover:brightness-90': !disabled,
        'brightness-70': disabled,
      }"
  >
    <slot />
  </button>
</template>

<style scoped>

</style>