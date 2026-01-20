<script setup lang="ts">
// Redirect /plugins to /discover/plugins with query params preserved
const route = useNativeRoute()

await navigateTo(
  {
    path: '/discover/plugins',
    query: route.query,
  },
  { redirectCode: 301, replace: true },
)
</script>

<template>
  <div />
</template>
