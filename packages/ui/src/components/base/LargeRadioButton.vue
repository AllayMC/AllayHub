<template>
  <button
    role="radio"
    :aria-checked="selected"
    :aria-disabled="disabled"
    class="flex cursor-pointer gap-2 rounded-xl border-0 border-2 border-solid border-button-bg px-4 py-3 text-left font-medium transition-all"
    :class="
      (selected
        ? 'bg-button-bg text-contrast'
        : 'bg-transparent text-primary') +
      (disabled
        ? ' opacity-50'
        : ' hover:bg-button-bg hover:brightness-[--hover-brightness] active:scale-[0.98]')
    "
    :disabled="disabled"
    @click="emit('select')"
  >
    <RadioButtonCheckedIcon
      v-if="selected"
      class="h-5 w-5 shrink-0 text-brand"
      aria-hidden="true"
    />
    <RadioButtonIcon v-else class="h-5 w-5 shrink-0" aria-hidden="true" />
    <slot />
  </button>
</template>
<script setup lang="ts" generic="T">
import { RadioButtonCheckedIcon, RadioButtonIcon } from '@modrinth/assets'

const emit = defineEmits<{
  (e: 'select'): void
}>()

withDefaults(
  defineProps<{
    selected: boolean
    disabled?: boolean
  }>(),
  {
    disabled: false,
  },
)
</script>
