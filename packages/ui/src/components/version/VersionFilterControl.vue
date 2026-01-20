<template>
  <div class="experimental-styles-within flex flex-col gap-3">
    <div class="flex flex-wrap items-center gap-2">
      <ManySelect
        v-model="selectedChannels"
        :options="filterOptions.channel"
        :dropdown-id="`${baseId}-channel`"
        @change="updateFilters"
      >
        <FilterIcon class="h-5 w-5 text-secondary" />
        Channels
        <template #option="{ option }">
          {{ option === 'release' ? 'Release' : 'Beta' }}
        </template>
      </ManySelect>
    </div>
    <div class="flex flex-wrap items-center gap-1 empty:hidden">
      <TagItem
        v-if="selectedChannels.length > 1"
        class="transition-transform active:scale-[0.95]"
        :action="clearFilters"
      >
        <XCircleIcon />
        Clear all filters
      </TagItem>
      <TagItem
        v-for="channel in selectedChannels"
        :key="`remove-filter-${channel}`"
        :style="`--_color: var(--color-${channel === 'beta' ? 'orange' : 'green'});--_bg-color: var(--color-${channel === 'beta' ? 'orange' : 'green'}-highlight)`"
        :action="() => toggleFilter('channel', channel)"
      >
        <XIcon />
        {{ channel.slice(0, 1).toUpperCase() + channel.slice(1) }}
      </TagItem>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FilterIcon, XCircleIcon, XIcon } from '@modrinth/assets'
import { computed, ref } from 'vue'
import { useRoute } from 'vue-router'

import TagItem from '../base/TagItem.vue'
import { ManySelect } from '../index'

const props = defineProps<{
  versions: { prerelease: boolean }[]
  baseId?: string
}>()

const emit = defineEmits(['update:query'])

const allChannels = ref(['release', 'beta'])

const route = useRoute()

type FilterType = 'channel'
type Filter = string

const filterOptions = computed(() => {
  const filters: Record<FilterType, Filter[]> = {
    channel: [],
  }

  const channelSet = new Set()

  for (const version of props.versions) {
    channelSet.add(version.prerelease ? 'beta' : 'release')
  }

  if (channelSet.size > 0) {
    filters.channel = Array.from(channelSet) as Filter[]
    filters.channel.sort(
      (a, b) => allChannels.value.indexOf(a) - allChannels.value.indexOf(b),
    )
  }

  return filters
})

const selectedChannels = ref<string[]>([])

selectedChannels.value = route.query.c ? getArrayOrString(route.query.c) : []

async function toggleFilter(type: FilterType, filter: Filter, bulk = false) {
  if (type === 'channel') {
    selectedChannels.value = selectedChannels.value.includes(filter)
      ? selectedChannels.value.filter((x) => x !== filter)
      : [...selectedChannels.value, filter]
  }
  if (!bulk) {
    updateFilters()
  }
}

async function clearFilters() {
  selectedChannels.value = []
  updateFilters()
}

function updateFilters() {
  emit('update:query', {
    c: selectedChannels.value,
    page: undefined,
  })
}

defineExpose({
  toggleFilter,
  selectedChannels,
})

function getArrayOrString(x: unknown): string[] {
  if (typeof x === 'string') {
    return [x]
  } else if (Array.isArray(x)) {
    return x.filter((v): v is string => typeof v === 'string')
  }
  return []
}
</script>
