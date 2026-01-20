<template>
  <div class="flex flex-col gap-1">
    <button
      v-for="(item, index) in items"
      :key="`radio-button-${index}`"
      class="flex w-fit cursor-pointer items-center gap-2 rounded-xl border-0 p-0 px-2 py-2 font-medium transition-all hover:bg-button-bg active:scale-95"
      :class="{
        'bg-button-bg text-contrast': selected === item,
        'bg-transparent text-primary': selected !== item,
      }"
      @click="selected = item"
    >
      <RadioButtonCheckedIcon
        v-if="selected === item"
        class="h-5 w-5 text-brand"
      />
      <RadioButtonIcon v-else class="h-5 w-5" />
      <slot :item="item" />
    </button>
  </div>
</template>
<script setup lang="ts" generic="T">
import { RadioButtonCheckedIcon, RadioButtonIcon } from '@modrinth/assets'
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    modelValue: T
    items: T[]
    forceSelection?: boolean
  }>(),
  {
    forceSelection: false,
  },
)

const emit = defineEmits(['update:modelValue'])

const selected = computed({
  get() {
    return props.modelValue
  },
  set(value) {
    emit('update:modelValue', value)
  },
})

if (props.items.length > 0 && props.forceSelection && !props.modelValue) {
  selected.value = props.items[0]
}
</script>
