<template>
  <div v-if="showWarnings" class="flex flex-col gap-3">
    <h2 class="m-0 text-lg">{{ formatMessage(messages.title) }}</h2>

    <!-- Warning: Outdated plugin (no compatible version with latest API) -->
    <div v-if="props.isOutdated" class="flex flex-col gap-1">
      <span class="flex items-center gap-1 font-semibold text-orange">
        <InfoIcon class="h-4 w-4" aria-hidden="true" />
        {{ formatMessage(messages.outdatedTitle) }}
      </span>
      <span class="text-sm text-secondary">{{
        formatMessage(messages.outdatedDescription)
      }}</span>
    </div>

    <!-- Warning: Uses Server API (like NMS, not guaranteed compatibility) -->
    <div v-if="props.usesServerApi" class="flex flex-col gap-1">
      <span class="flex items-center gap-1 font-semibold text-orange">
        <InfoIcon class="h-4 w-4" aria-hidden="true" />
        {{ formatMessage(messages.serverApiTitle) }}
      </span>
      <span class="text-sm text-secondary">{{
        formatMessage(messages.serverApiDescription)
      }}</span>
    </div>
  </div>
</template>
<script setup lang="ts">
import { InfoIcon } from '@modrinth/assets'
import { computed } from 'vue'

import { defineMessages, useVIntl } from '../../composables/i18n'

const { formatMessage } = useVIntl()

const props = defineProps<{
  isOutdated: boolean
  usesServerApi: boolean
}>()

// Show section only if there are warnings
const showWarnings = computed(() => props.isOutdated || props.usesServerApi)

const messages = defineMessages({
  title: {
    id: `project.about.compatibility.title`,
    defaultMessage: 'Compatibility',
  },
  outdatedTitle: {
    id: `project.about.compatibility.outdated.title`,
    defaultMessage: 'Outdated Plugin',
  },
  outdatedDescription: {
    id: `project.about.compatibility.outdated.description`,
    defaultMessage:
      'This plugin may not be compatible with the latest Allay API version. Please check for updates.',
  },
  serverApiTitle: {
    id: `project.about.compatibility.serverApi.title`,
    defaultMessage: 'Uses Server API',
  },
  serverApiDescription: {
    id: `project.about.compatibility.serverApi.description`,
    defaultMessage:
      'This plugin uses internal Server API which may break between versions.',
  },
})
</script>
