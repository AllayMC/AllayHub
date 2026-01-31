<template>
  <section class="normal-page__content">
    <div class="card dependencies-card">
      <h2 class="mb-4 text-lg font-bold">Dependencies</h2>
      <div
        v-for="dep in sortedDependencies"
        :key="dep.plugin_id"
        class="dependency-group"
      >
        <template v-if="resolvedDeps[dep.plugin_id]">
          <!-- Single match: direct link -->
          <NuxtLink
            v-if="resolvedDeps[dep.plugin_id].length === 1"
            :to="`/plugin/${resolvedDeps[dep.plugin_id][0].id}`"
            class="dependency"
          >
            <Avatar
              :src="resolvedDeps[dep.plugin_id][0].icon_url"
              alt="dependency-icon"
              size="sm"
            />
            <div class="info">
              <span class="project-title">
                {{ resolvedDeps[dep.plugin_id][0].name }}
              </span>
              <span class="dep-type" :class="dep.dependency_type">
                <template v-if="dep.version_range">
                  Version {{ dep.version_range }} is {{ dep.dependency_type }}
                </template>
                <template v-else>
                  {{ dep.dependency_type }}
                </template>
              </span>
            </div>
          </NuxtLink>

          <!-- Multiple matches: popup to choose -->
          <OverflowMenu
            v-else-if="resolvedDeps[dep.plugin_id].length > 1"
            :options="toMenuOptions(resolvedDeps[dep.plugin_id])"
            placement="bottom-start"
          >
            <template #default>
              <div class="dependency">
                <Avatar
                  :src="resolvedDeps[dep.plugin_id][0].icon_url"
                  alt="dependency-icon"
                  size="sm"
                />
                <div class="info">
                  <span class="project-title">
                    {{ dep.plugin_id }}
                    <span class="owner-hint">
                      ({{ resolvedDeps[dep.plugin_id].length }} matches)
                    </span>
                  </span>
                  <span class="dep-type" :class="dep.dependency_type">
                    <template v-if="dep.version_range">
                      Version {{ dep.version_range }} is
                      {{ dep.dependency_type }}
                    </template>
                    <template v-else>
                      {{ dep.dependency_type }}
                    </template>
                  </span>
                </div>
              </div>
            </template>
            <template
              v-for="resolved in resolvedDeps[dep.plugin_id]"
              :key="resolved.id"
              #[slotKey(resolved.id)]
            >
              <Avatar
                :src="resolved.icon_url"
                alt="dependency-icon"
                size="xs"
              />
              {{ resolved.name }}
              <span class="owner-hint">({{ resolved.owner }})</span>
            </template>
          </OverflowMenu>
        </template>
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
import { Avatar, OverflowMenu } from '@modrinth/ui'
import { computed, reactive, watch } from 'vue'
import { usePlugin, findPluginIdsByName } from '~/composables/usePlugins'

const props = defineProps<{
  project: AllayIndex.ProjectView
  versions: AllayIndex.Version[]
  members: AllayIndex.MemberView[]
  organization: object | null
}>()

interface ResolvedDep {
  id: string
  name: string
  owner: string
  icon_url?: string
}

// Sort dependencies: required first, then optional, then others
const sortedDependencies = computed(() => {
  const deps = props.project.dependencies || []
  const order = ['required', 'optional', 'incompatible', 'embedded']
  return [...deps].sort(
    (a, b) =>
      order.indexOf(a.dependency_type) - order.indexOf(b.dependency_type),
  )
})

// Store resolved dependency data (supports multiple matches per dep)
const resolvedDeps = reactive<Record<string, ResolvedDep[]>>({})

// Slot-safe key: replace '/' with '--' for Vue dynamic slot names
function slotKey(id: string) {
  return id.replace(/\//g, '--')
}

// Convert resolved deps to OverflowMenu options
function toMenuOptions(deps: ResolvedDep[]) {
  return deps.map((d) => ({
    id: slotKey(d.id),
    link: `/plugin/${d.id}`,
  }))
}

// Load each dependency's plugin data, expanding 1:n matches
async function loadDependencyPlugins() {
  const deps = props.project.dependencies || []
  for (const dep of deps) {
    if (resolvedDeps[dep.plugin_id]) continue

    const matchedIds = findPluginIdsByName(dep.plugin_id)
    const entries: ResolvedDep[] = []

    for (const fullId of matchedIds) {
      if (fullId.includes('/')) {
        const { data } = await usePlugin(fullId)
        const owner = fullId.split('/')[0]
        entries.push({
          id: fullId,
          name: data.value?.name || dep.plugin_id,
          owner,
          icon_url: data.value?.icon_url,
        })
      } else {
        entries.push({
          id: dep.plugin_id,
          name: dep.plugin_id,
          owner: '',
        })
      }
    }

    resolvedDeps[dep.plugin_id] = entries
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
  .dependency-group + .dependency-group {
    margin-top: var(--spacing-card-xs);
  }

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

        .owner-hint {
          font-weight: normal;
          color: var(--color-text-secondary);
          font-size: var(--font-size-sm);
        }
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
