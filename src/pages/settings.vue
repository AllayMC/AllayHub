<template>
  <div>
    <div class="normal-page no-sidebar">
      <h1>{{ formatMessage(commonMessages.settingsLabel) }}</h1>
    </div>
    <div class="normal-page">
      <div class="normal-page__sidebar">
        <NavStack
          :items="[
            { type: 'heading', label: 'Display' },
            {
              link: '/settings',
              label: formatMessage(commonSettingsMessages.appearance),
              icon: PaintbrushIcon,
            },
            {
              link: '/settings/language',
              label: formatMessage(commonSettingsMessages.language),
              icon: LanguagesIcon,
              badge: `${formatMessage(commonMessages.beta)}`,
            },
          ]"
        />
      </div>
      <div class="normal-page__content mt-3 lg:mt-0">
        <NuxtPage :route="route" />
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { LanguagesIcon, PaintbrushIcon } from '@modrinth/assets'
import { commonMessages, commonSettingsMessages, useVIntl } from '@modrinth/ui'

import NavStack from '~/components/ui/NavStack.vue'

const { formatMessage } = useVIntl()

const route = useNativeRoute()
</script>
