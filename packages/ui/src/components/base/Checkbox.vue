<template>
  <button
    class="checkbox-outer group m-0 flex items-center gap-3 border-none bg-transparent p-0 text-contrast outline-offset-4"
    :disabled="disabled"
    :class="
      disabled
        ? 'cursor-not-allowed opacity-50'
        : 'cursor-pointer hover:brightness-[--hover-brightness] focus-visible:brightness-[--hover-brightness]'
    "
    :aria-label="description || label"
    :aria-checked="modelValue"
    role="checkbox"
    @click="toggle"
  >
    <span
      class="flex h-5 w-5 items-center justify-center rounded-md border-[1px] border-solid"
      :class="
        (modelValue
          ? 'border-button-border bg-brand text-brand-inverted'
          : 'border-surface-5 bg-surface-2') +
        (disabled ? '' : ' checkbox-shadow group-active:scale-95')
      "
    >
      <MinusIcon v-if="indeterminate" aria-hidden="true" stroke-width="3" />
      <CheckIcon v-else-if="modelValue" aria-hidden="true" stroke-width="3" />
    </span>
    <!-- aria-hidden is set so screenreaders only use the <button>'s aria-label -->
    <span v-if="label" aria-hidden="true">
      {{ label }}
    </span>
    <slot v-else />
  </button>
</template>
<script setup lang="ts">
import { CheckIcon, MinusIcon } from '@modrinth/assets'

const emit = defineEmits<{
  'update:modelValue': [boolean]
}>()

const props = withDefaults(
  defineProps<{
    label?: string
    disabled?: boolean
    description?: string
    modelValue: boolean
    clickEvent?: () => void
    indeterminate?: boolean
  }>(),
  {
    label: '',
    disabled: false,
    description: '',
    modelValue: false,
    clickEvent: () => {},
    indeterminate: false,
  },
)

function toggle() {
  if (!props.disabled) {
    emit('update:modelValue', !props.modelValue)
  }
}
</script>

<style lang="scss" scoped>
.checkbox-shadow {
  box-shadow: 1px 1px 2px 0 rgba(0, 0, 0, 0.08);
}
</style>
