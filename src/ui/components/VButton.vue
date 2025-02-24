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
      class="py-2 px-4 rounded-xl transition duration-100"
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