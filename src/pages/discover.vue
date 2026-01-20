<script setup lang="ts">
import { commonProjectTypeCategoryMessages, useVIntl } from '@modrinth/ui'

import NavTabs from '~/components/ui/NavTabs.vue'

const { formatMessage } = useVIntl()

const flags = useFeatureFlags()
const cosmetics = useCosmetics()
const route = useRoute()

const allowTabChanging = computed(() => !route.query.sid)

const selectableProjectTypes = [
  {
    label: formatMessage(commonProjectTypeCategoryMessages.plugin),
    href: `/discover/plugins`,
    type: 'plugins',
  },
]
</script>
<template>
  <div
    class="new-page sidebar"
    :class="{ 'alt-layout': !cosmetics.rightSearchLayout }"
  >
    <section class="normal-page__header mb-4 flex flex-col gap-4">
      <div id="discover-header-prefix" class="empty:hidden"></div>
      <NavTabs
        v-if="!flags.projectTypesPrimaryNav && allowTabChanging"
        :links="selectableProjectTypes"
        class="hidden md:flex"
      />
    </section>
    <NuxtPage />
  </div>
</template>
