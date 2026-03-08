<template>
  <div v-if="showWarnings" class="flex flex-col gap-3">
    <h2 class="m-0 text-lg">{{ formatMessage(messages.title) }}</h2>

    <div v-if="props.dependsOnNukkitMot" class="flex flex-col gap-1">
      <span class="flex items-center gap-1 font-semibold text-orange">
        <InfoIcon class="h-4 w-4" aria-hidden="true" />
        {{ formatMessage(messages.nukkitMotDependencyTitle) }}
      </span>
      <span class="text-sm text-secondary">{{
        formatMessage(messages.nukkitMotDependencyDescription)
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
  dependsOnNukkitMot: boolean
}>()

const showWarnings = computed(() => props.dependsOnNukkitMot)

const messages = defineMessages({
  title: {
    id: `project.about.compatibility.title`,
    defaultMessage: 'Compatibility',
  },
  nukkitMotDependencyTitle: {
    id: `project.about.compatibility.nukkitMotDependency.title`,
    defaultMessage: 'Depends on Nukkit-MOT',
  },
  nukkitMotDependencyDescription: {
    id: `project.about.compatibility.nukkitMotDependency.description`,
    defaultMessage:
      'This plugin declares a dependency on Nukkit-MOT in plugin.yml.',
  },
})
</script>
