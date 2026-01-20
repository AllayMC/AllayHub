<template>
  <NuxtLayout>
    <NuxtRouteAnnouncer />
    <ModrinthLoadingIndicator />
    <NuxtPage />
  </NuxtLayout>
</template>
<script setup lang="ts">
import { providePageContext } from '@modrinth/ui'
import ModrinthLoadingIndicator from '~/components/ui/modrinth-loading-indicator.ts'
providePageContext({
  hierarchicalSidebarAvailable: ref(false),
  showAds: ref(false),
})
</script>
