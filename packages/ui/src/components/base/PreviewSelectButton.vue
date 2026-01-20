<script setup lang="ts">
import { RadioButtonCheckedIcon, RadioButtonIcon } from '@modrinth/assets'

withDefaults(
  defineProps<{
    checked: boolean
  }>(),
  {
    checked: false,
  },
)
</script>
<template>
  <div class="" role="button" @click="() => {}">
    <slot name="preview" />
    <div>
      <RadioButtonIcon v-if="!checked" class="h-4 w-4" />
      <RadioButtonCheckedIcon v-else class="h-4 w-4" />
      <slot />
    </div>
  </div>
</template>
