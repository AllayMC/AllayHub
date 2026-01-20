<template>
  <button
    v-if="action"
    class="inline-flex cursor-pointer items-center gap-1 rounded-full border-none bg-[--_bg-color,var(--color-button-bg)] px-2 py-1 text-sm font-semibold leading-none text-[--_color,var(--color-secondary)] transition-transform hover:underline active:scale-[0.95] [&>svg]:h-4 [&>svg]:w-4 [&>svg]:shrink-0"
    @click="action"
  >
    <slot />
  </button>
  <div
    v-else
    class="inline-flex items-center gap-1 rounded-full bg-[--_bg-color,var(--color-button-bg)] px-2 py-1 text-sm font-semibold leading-none text-[--_color,var(--color-secondary)] [&>svg]:h-4 [&>svg]:w-4 [&>svg]:shrink-0"
  >
    <slot />
  </div>
</template>
<script setup lang="ts">
defineProps<{
  action?: (event: MouseEvent) => void
}>()
</script>
