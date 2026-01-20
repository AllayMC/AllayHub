<template>
  <AutoLink
    :to="to"
    class="group mb-3 flex w-fit items-center gap-1 text-lg font-bold leading-none text-primary hover:underline"
  >
    <slot />
    <ChevronRightIcon
      class="h-5 w-5 stroke-[3px] transition-transform group-hover:translate-x-1 group-hover:text-brand"
    />
  </AutoLink>
</template>

<script setup lang="ts">
import { ChevronRightIcon } from '@modrinth/assets'

import AutoLink from './AutoLink.vue'

defineProps<{
  to: unknown
}>()
</script>
