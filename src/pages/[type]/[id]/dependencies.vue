<template>
  <section class="normal-page__content">
    <div class="card dependencies-card">
      <h2 class="mb-4 text-lg font-bold">Dependencies</h2>
      <div
        v-for="dep in sortedDependencies"
        :key="dep.plugin_id"
        class="dependency"
      >
        <Avatar
          :src="dependencyPlugins[dep.plugin_id]?.icon_url"
          alt="dependency-icon"
          size="sm"
        />
        <NuxtLink :to="`/plugin/${dep.plugin_id}`" class="info">
          <span class="project-title">
            {{ dependencyPlugins[dep.plugin_id]?.name || dep.plugin_id }}
          </span>
          <span class="dep-type" :class="dep.dependency_type">
            <template v-if="dep.version_range">
              Version {{ dep.version_range }} is {{ dep.dependency_type }}
            </template>
            <template v-else>
              {{ dep.dependency_type }}
            </template>
          </span>
        </NuxtLink>
      </div>
      <p
        v-if="!project.dependencies || project.dependencies.length === 0"
        class="text-secondary"
      >
        No dependencies.
      </p>
    </div>
  </section>
</template>

<script setup lang="ts">
import { Avatar } from '@modrinth/ui'
import { computed, reactive, watch } from 'vue'
import { usePlugin } from '~/composables/usePlugins'

const props = defineProps<{
  project: AllayIndex.ProjectView
  versions: AllayIndex.Version[]
  members: AllayIndex.MemberView[]
  organization: object | null
}>()

// Sort dependencies: required first, then optional, then others
const sortedDependencies = computed(() => {
  const deps = props.project.dependencies || []
  const order = ['required', 'optional', 'incompatible', 'embedded']
  return [...deps].sort(
    (a, b) =>
      order.indexOf(a.dependency_type) - order.indexOf(b.dependency_type),
  )
})

// Store loaded dependency plugin data
const dependencyPlugins = reactive<
  Record<string, { name: string; icon_url?: string } | null>
>({})

// Load each dependency's plugin data
async function loadDependencyPlugins() {
  const deps = props.project.dependencies || []
  for (const dep of deps) {
    if (!dependencyPlugins[dep.plugin_id]) {
      const { data } = await usePlugin(dep.plugin_id)
      if (data.value) {
        dependencyPlugins[dep.plugin_id] = {
          name: data.value.name,
          icon_url: data.value.icon_url,
        }
      }
    }
  }
}

// Load on mount and when dependencies change
watch(
  () => props.project.dependencies,
  () => loadDependencyPlugins(),
  { immediate: true },
)
</script>

<style lang="scss" scoped>
.dependencies-card {
  .dependency {
    align-items: center;
    display: flex;
    gap: var(--spacing-card-sm);
    padding: var(--spacing-card-sm);
    border-radius: var(--size-rounded-sm);
    transition: background-color 0.1s ease-in-out;

    &:hover {
      background-color: var(--color-button-bg);
    }

    .info {
      display: flex;
      flex-direction: column;
      gap: var(--spacing-card-xs);
      text-decoration: none;
      color: inherit;

      .project-title {
        font-weight: bold;
      }

      .dep-type {
        color: var(--color-text-secondary);
        font-size: var(--font-size-sm);

        &.required {
          color: var(--color-brand);
        }

        &.optional {
          color: var(--color-text-secondary);
        }

        &.incompatible {
          color: var(--color-red);
        }

        &::first-letter {
          text-transform: capitalize;
        }
      }
    }
  }
}
</style>
