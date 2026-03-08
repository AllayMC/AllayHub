<template>
  <section class="normal-page__content">
    <div v-if="project.body" class="card">
      <ProjectPageDescription :description="project.body" />
    </div>
    <p v-else class="ml-2">
      No description has been provided for this project yet.
    </p>
  </section>
</template>

<script setup lang="ts">
import { ProjectPageDescription } from '@modrinth/ui'

defineProps<{
  project: AllayIndex.ProjectView
  versions: AllayIndex.Version[]
  members: AllayIndex.MemberView[]
  organization: object | null
}>()
</script>
