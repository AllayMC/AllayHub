<template>
  <section class="normal-page__content">
    <div v-if="project.body" class="card">
      <ProjectPageDescription :description="project.body" />
    </div>
    <p v-else class="ml-2">
      No description provided. Visit
      <NuxtLink :to="`${route.fullPath}/settings/description`">
        <span class="font-medium text-green hover:underline"
          >project settings</span
        >
        to
      </NuxtLink>
      add your description.
    </p>
  </section>
</template>

<script setup lang="ts">
import { ProjectPageDescription } from '@modrinth/ui'

const route = useRoute()

defineProps<{
  project: AllayIndex.ProjectView
  versions: AllayIndex.Version[]
  members: AllayIndex.MemberView[]
  organization: object | null
}>()
</script>
