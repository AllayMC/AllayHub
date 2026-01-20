<template>
  <div
    v-if="(showAllOptions && options.length > 0) || options.length > 1"
    class="flex flex-wrap items-center gap-1"
  >
    <FilterIcon class="mr-1 h-5 w-5 text-secondary" />
    <button
      v-for="filter in options"
      :key="`filter-${filter.id}`"
      :class="`cursor-pointer rounded-full border-none px-2 py-1 font-semibold leading-none transition-all duration-100 active:scale-[0.97] ${selectedFilters.includes(filter.id) ? 'bg-brand-highlight text-brand' : 'bg-bg-raised text-secondary'}`"
      @click="toggleFilter(filter.id)"
    >
      {{ formatMessage(filter.message) }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { FilterIcon } from '@modrinth/assets'
import { watch } from 'vue'

import { type MessageDescriptor, useVIntl } from '../../composables/i18n'

const { formatMessage } = useVIntl()

export type FilterBarOption = {
  id: string
  message: MessageDescriptor
}

const selectedFilters = defineModel<string[]>({ required: true })

const props = defineProps<{
  options: FilterBarOption[]
  showAllOptions?: boolean
}>()

watch(
  () => props.options,
  () => {
    for (let i = 0; i < selectedFilters.value.length; i++) {
      const option = selectedFilters.value[i]
      if (!props.options.some((x) => x.id === option)) {
        selectedFilters.value.splice(i, 1)
      }
    }
  },
)

function toggleFilter(option: string) {
  if (selectedFilters.value.includes(option)) {
    selectedFilters.value.splice(selectedFilters.value.indexOf(option), 1)
  } else {
    selectedFilters.value.push(option)
  }
}
</script>
