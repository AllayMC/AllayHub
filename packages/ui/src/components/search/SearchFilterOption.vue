<template>
  <div class="search-filter-option group flex items-center gap-1">
    <button
      :class="`flex !w-full cursor-pointer items-center gap-2 truncate rounded-xl border-none px-2 py-2 text-sm font-semibold transition-all hover:text-contrast focus-visible:text-contrast active:scale-[0.98] [@media(hover:hover)]:py-1 ${included ? 'bg-brand-highlight text-contrast hover:brightness-125' : excluded ? 'bg-highlight-red text-contrast hover:brightness-125' : 'bg-transparent text-secondary hover:bg-button-bg focus-visible:bg-button-bg [&>svg.check-icon]:hover:text-brand [&>svg.check-icon]:focus-visible:text-brand'}`"
      @click="() => emit('toggle', option)"
    >
      <slot> </slot>
      <BanIcon
        v-if="excluded"
        :class="`filter-action-icon ml-auto h-4 w-4 shrink-0 transition-opacity group-hover:opacity-100 ${excluded ? '' : '[@media(hover:hover)]:opacity-0'}`"
        aria-hidden="true"
      />
      <CheckIcon
        v-else
        :class="`filter-action-icon check-icon ml-auto h-4 w-4 shrink-0 transition-opacity group-hover:opacity-100 ${included ? '' : '[@media(hover:hover)]:opacity-0'}`"
        aria-hidden="true"
      />
    </button>
    <div
      v-if="supportsNegativeFilter && !excluded"
      class="h-[1.75rem] w-px bg-button-bg [@media(hover:hover)]:contents"
      :class="{ 'opacity-0': included }"
    ></div>
    <button
      v-if="supportsNegativeFilter && !excluded"
      v-tooltip="excluded ? 'Remove exclusion' : 'Exclude'"
      class="flex cursor-pointer items-center justify-center gap-2 rounded-xl border-none bg-transparent px-2 py-1 text-sm font-semibold text-secondary transition-all hover:bg-button-bg hover:text-red focus-visible:bg-button-bg focus-visible:text-red active:scale-[0.96] [@media(hover:hover)]:opacity-0"
      @click="() => emit('toggleExclude', option)"
    >
      <BanIcon class="filter-action-icon h-4 w-4" aria-hidden="true" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { BanIcon, CheckIcon } from '@modrinth/assets'

import type { FilterOption } from '../../utils/search'

withDefaults(
  defineProps<{
    option: FilterOption
    included: boolean
    excluded: boolean
    supportsNegativeFilter?: boolean
  }>(),
  {
    supportsNegativeFilter: false,
  },
)

const emit = defineEmits<{
  toggle: [option: FilterOption]
  toggleExclude: [option: FilterOption]
}>()
</script>
<style scoped lang="scss">
.search-filter-option:hover,
.search-filter-option:has(button:focus-visible) {
  button,
  .filter-action-icon {
    opacity: 1;
  }
}
</style>
