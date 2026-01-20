<script setup lang="ts">
import type { VersionChannel } from '@modrinth/utils'

import { defineMessages, useVIntl } from '../../composables/i18n'

const { formatMessage } = useVIntl()

withDefaults(
  defineProps<{
    channel: VersionChannel
    large?: boolean
  }>(),
  {
    large: false,
  },
)

const messages = defineMessages({
  releaseSymbol: {
    id: 'project.versions.channel.release.symbol',
    defaultMessage: 'R',
  },
  betaSymbol: {
    id: 'project.versions.channel.beta.symbol',
    defaultMessage: 'B',
  },
  alphaSymbol: {
    id: 'project.versions.channel.alpha.symbol',
    defaultMessage: 'A',
  },
})
</script>

<template>
  <div
    :class="`flex ${large ? 'h-[2.625rem] w-[2.625rem] text-lg' : 'h-9 w-9 text-sm'} items-center justify-center rounded-full font-bold ${channel === 'release' ? 'bg-bg-green text-brand-green' : channel === 'beta' ? 'bg-bg-orange text-brand-orange' : 'bg-bg-red text-brand-red'}`"
  >
    {{ channel ? formatMessage(messages[`${channel}Symbol`]) : '?' }}
  </div>
</template>
