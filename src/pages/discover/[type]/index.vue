<script setup lang="ts">
import {
  FilterIcon,
  GridIcon,
  ImageIcon,
  ListIcon,
  SearchIcon,
  XIcon,
} from '@modrinth/assets'
import { defineMessages, useVIntl } from '@modrinth/ui'
import {
  Button,
  ButtonStyled,
  DropdownSelect,
  Pagination,
  SearchSidebarFilter,
  SearchFilterControl,
} from '@modrinth/ui'
import type { FilterType, FilterValue } from '@modrinth/ui'
import { capitalizeString, cycleValue } from '@modrinth/utils'
import { useThrottleFn } from '@vueuse/core'
import semver from 'semver'
import { computed, watch } from 'vue'

import LogoAnimated from '~/components/brand/LogoAnimated.vue'
import ProjectCard from '~/components/ui/ProjectCard.vue'
import type { DisplayLocation, DisplayMode } from '~/plugins/cosmetics.ts'
import {
  usePluginSearch,
  type SortOption,
  getTotalPages,
  useCategories,
  useApiVersions,
} from '~/composables/usePlugins'

const { formatMessage } = useVIntl()

const filtersMenuOpen = ref(false)

const route = useNativeRoute()
const router = useNativeRouter()

const cosmetics = useCosmetics()
const flags = useFeatureFlags()

// Use new index composables
const { search, isSearching } = usePluginSearch()
const { data: categoriesData } = useCategories()
const { data: apiVersionsData } = useApiVersions()

// Search state
const query = ref((route.query.q as string) ?? '')
const currentPage = ref(Number(route.query.p) || 1)
const perPage = ref(Number(route.query.pp) || 20)
const sortType = ref<SortOption>((route.query.s as SortOption) ?? 'downloads')

// Filter state
const currentFilters = ref<FilterValue[]>([])
const toggledGroups = ref<string[]>([])
const overriddenProvidedFilterTypes = ref<string[]>([])
const providedFilters = ref<FilterValue[]>([])

// Computed selected categories from filters (for search function)
const selectedCategories = computed(() =>
  currentFilters.value
    .filter((f) => f.type === 'categories' && !f.negative)
    .map((f) => f.option),
)

// Computed selected API major version from filters (max of selected versions)
const selectedApiMajor = computed(() => {
  const versions = currentFilters.value
    .filter((f) => f.type === 'api_version' && !f.negative)
    .map((f) => semver.parse(f.option)?.major ?? 0)
  return versions.length > 0 ? Math.max(...versions) : undefined
})

// Computed selected license from filters
const selectedLicense = computed(() => {
  const licenseFilter = currentFilters.value.find(
    (f) => f.type === 'license' && !f.negative,
  )
  return licenseFilter?.option as 'open-source' | 'closed-source' | undefined
})

// Sort options
const sortOptions: { value: SortOption; label: string }[] = [
  { value: 'downloads', label: 'Downloads' },
  { value: 'stars', label: 'Stars' },
  { value: 'updated', label: 'Updated' },
  { value: 'newest', label: 'Newest' },
]

// Display modes
const resultsDisplayLocation = computed<DisplayLocation>(() => 'plugin')
const resultsDisplayMode = computed<DisplayMode>(
  () =>
    cosmetics.value.searchDisplayMode[resultsDisplayLocation.value] ?? 'list',
)

const maxResultsForView = ref<Record<DisplayMode, number[]>>({
  list: [5, 10, 15, 20, 50, 100],
  grid: [6, 12, 18, 24, 48, 96],
  gallery: [6, 10, 16, 20, 50, 100],
})

const currentMaxResultsOptions = computed(
  () => maxResultsForView.value[resultsDisplayMode.value] ?? [20],
)

// Search results
const searchResults = ref<AllayIndex.PluginSummary[]>([])
const totalResults = ref(0)
const searchLoading = computed(() => isSearching.value)

// Page count
const pageCount = computed(() =>
  getTotalPages(totalResults.value, perPage.value),
)

// Categories for filter sidebar
const categories = computed(() => categoriesData.value?.categories ?? [])

// API versions for filter sidebar
const apiVersions = computed(() => apiVersionsData.value?.versions ?? [])

// Load category icons from assets
const categoryIcons = import.meta.glob<string>(
  '~/assets/images/category/*.svg',
  {
    query: '?raw',
    import: 'default',
    eager: true,
  },
)

// Get icon SVG content by category ID
function getCategoryIcon(categoryId: string): string | undefined {
  const key = Object.keys(categoryIcons).find((k) =>
    k.includes(`/${categoryId}.svg`),
  )
  return key ? categoryIcons[key] : undefined
}

// Filter types for SearchSidebarFilter
const filters = computed<FilterType[]>(() => {
  const filterTypes: FilterType[] = []

  // Categories filter
  if (categories.value.length > 0) {
    filterTypes.push({
      id: 'categories',
      formatted_name: 'Categories',
      supported_project_types: ['plugin'],
      display: 'all',
      query_param: 'c',
      supports_negative_filter: true,
      searchable: false,
      options: categories.value.map((cat) => ({
        id: cat.id,
        formatted_name: cat.name,
        icon: getCategoryIcon(cat.id),
        value: `categories:${cat.id}`,
        method: 'and' as const,
      })),
    })
  }

  // API Version filter
  if (apiVersions.value.length > 0) {
    filterTypes.push({
      id: 'api_version',
      formatted_name: 'API Version',
      supported_project_types: ['plugin'],
      display: 'scrollable',
      query_param: 'v',
      supports_negative_filter: false,
      searchable: false,
      options: apiVersions.value.map((ver) => ({
        id: ver.version,
        formatted_name: ver.version,
        value: `api_version:${ver.version}`,
        method: 'or' as const,
      })),
    })
  }

  // License filter
  filterTypes.push({
    id: 'license',
    formatted_name: 'License',
    supported_project_types: ['plugin'],
    display: 'all',
    query_param: 'l',
    supports_negative_filter: false,
    searchable: false,
    options: [
      {
        id: 'open-source',
        formatted_name: 'Open Source',
        method: 'or' as const,
        value: 'license:open-source',
      },
      {
        id: 'closed-source',
        formatted_name: 'Closed Source',
        method: 'or' as const,
        value: 'license:closed-source',
      },
    ],
  })

  return filterTypes
})

const messages = defineMessages({
  searchPlaceholder: {
    id: 'discover.search.placeholder',
    defaultMessage: 'Search plugins...',
  },
  noResults: {
    id: 'discover.search.no-results',
    defaultMessage: 'No results found for your query!',
  },
  sortBy: {
    id: 'discover.search.sort-by',
    defaultMessage: 'Sort by',
  },
  view: {
    id: 'discover.search.view',
    defaultMessage: 'View',
  },
  filters: {
    id: 'discover.search.filters',
    defaultMessage: 'Filters',
  },
  categories: {
    id: 'discover.search.categories',
    defaultMessage: 'Categories',
  },
})

// Build search filters from current state
function buildSearchFilters() {
  return {
    query: query.value || undefined,
    categories:
      selectedCategories.value.length > 0
        ? selectedCategories.value
        : undefined,
    apiMajor: selectedApiMajor.value,
    license: selectedLicense.value,
  }
}

// Perform search
async function performSearch() {
  const filters = buildSearchFilters()

  const results = await search(filters, {
    sort: sortType.value,
    page: currentPage.value,
    perPage: perPage.value,
  })
  searchResults.value = results

  // Get total count for pagination
  const allResults = await search(filters, { limit: 10000 })
  totalResults.value = allResults.length
}

function scrollToTop(behavior: ScrollBehavior = 'smooth') {
  window.scrollTo({ top: 0, behavior })
}

function updateSearchResults(pageNumber: number = 1, resetScroll = true) {
  currentPage.value = pageNumber
  if (resetScroll) {
    scrollToTop()
  }

  performSearch()

  // Update URL params
  if (import.meta.client) {
    const params: Record<string, string> = {}
    if (query.value) params.q = query.value
    if (currentPage.value > 1) params.p = String(currentPage.value)
    if (sortType.value !== 'downloads') params.s = sortType.value
    if (perPage.value !== 20) params.pp = String(perPage.value)
    if (selectedCategories.value.length > 0)
      params.c = selectedCategories.value.join(',')

    router.replace({ path: route.path, query: params })
  }
}

// Watch for filter changes
watch(
  [currentFilters],
  () => {
    updateSearchResults(1, false)
  },
  { deep: true },
)

const throttledSearch = useThrottleFn(() => updateSearchResults(), 500, true)

function cycleSearchDisplayMode() {
  const modes: DisplayMode[] = ['list', 'grid', 'gallery']
  cosmetics.value.searchDisplayMode[resultsDisplayLocation.value] = cycleValue(
    cosmetics.value.searchDisplayMode[resultsDisplayLocation.value],
    modes,
  )
  setClosestMaxResults()
}

function setClosestMaxResults() {
  const maxResultsOptions = maxResultsForView.value[
    resultsDisplayMode.value
  ] ?? [20]
  const currentMax = perPage.value
  if (!maxResultsOptions.includes(currentMax)) {
    perPage.value = maxResultsOptions.reduce((prev: number, curr: number) => {
      return Math.abs(curr - currentMax) <= Math.abs(prev - currentMax)
        ? curr
        : prev
    })
  }
}

// Parse filter string like "categories:management" or "g=categories:management"
function parseFilterString(
  filterStr: string,
): { type: string; option: string } | null {
  const match = filterStr.match(/^(\w+):(.+)$/)
  if (match) {
    return { type: match[1], option: match[2] }
  }
  return null
}

// Initialize search on mount
onMounted(() => {
  // Parse URL params for categories
  if (route.query.c) {
    const categoryIds = (route.query.c as string).split(',')
    categoryIds.forEach((catId) => {
      currentFilters.value.push({
        type: 'categories',
        option: catId,
        negative: false,
      })
    })
  }
  // Parse URL params for API versions
  if (route.query.v) {
    const versions = (route.query.v as string).split(',')
    versions.forEach((ver) => {
      currentFilters.value.push({
        type: 'api_version',
        option: ver,
        negative: false,
      })
    })
  }
  // Parse URL params for license
  if (route.query.l === 'true') {
    currentFilters.value.push({
      type: 'license',
      option: 'open_source',
      negative: false,
    })
  }
  // Parse filter params (f= or g=)
  const filterParam = route.query.f || route.query.g
  if (filterParam) {
    const filterStr = filterParam as string
    const parsed = parseFilterString(filterStr)
    if (parsed) {
      currentFilters.value.push({
        type: parsed.type,
        option: parsed.option,
        negative: false,
      })
    }
  }
  performSearch()
})

// SEO
const ogTitle = computed(
  () => `Search plugins${query.value ? ' | ' + query.value : ''}`,
)
const description = computed(
  () =>
    `Search and browse AllayMC plugins on AllayHub with instant, accurate search results. Our filters help you quickly find the best plugins for your server.`,
)

useSeoMeta({
  description,
  ogTitle,
  ogDescription: description,
})
</script>
<template>
  <Teleport v-if="flags.searchBackground" to="#absolute-background-teleport">
    <div class="search-background"></div>
  </Teleport>

  <aside
    :class="{
      'normal-page__sidebar': true,
    }"
    aria-label="Filters"
  >
    <div v-if="filtersMenuOpen" class="fixed inset-0 z-40 bg-bg"></div>
    <div
      class="flex flex-col gap-3"
      :class="{
        'fixed inset-0 z-50 m-4 mb-0 overflow-auto rounded-t-3xl bg-bg-raised':
          filtersMenuOpen,
      }"
    >
      <div
        v-if="filtersMenuOpen"
        class="sticky top-0 z-10 mx-1 flex items-center justify-between gap-3 border-0 border-b-[1px] border-solid border-divider bg-bg-raised px-6 py-4"
      >
        <h3 class="m-0 text-lg text-contrast">
          {{ formatMessage(messages.filters) }}
        </h3>
        <ButtonStyled circular>
          <button
            @click="
              () => {
                filtersMenuOpen = false
                scrollToTop('instant')
              }
            "
          >
            <XIcon />
          </button>
        </ButtonStyled>
      </div>
      <!-- Filter sidebar using SearchSidebarFilter -->
      <template v-for="filter in filters" :key="`filter-${filter.id}`">
        <SearchSidebarFilter
          v-model:selected-filters="currentFilters"
          v-model:toggled-groups="toggledGroups"
          v-model:overridden-provided-filter-types="
            overriddenProvidedFilterTypes
          "
          :provided-filters="providedFilters"
          :filter-type="filter"
          :class="
            filtersMenuOpen
              ? 'border-0 border-b-[1px] border-solid border-divider last:border-b-0'
              : 'card-shadow rounded-2xl bg-bg-raised'
          "
          button-class="button-animation flex flex-col gap-1 px-6 py-4 w-full bg-transparent cursor-pointer border-none"
          content-class="mb-4 mx-3"
          inner-panel-class="p-1"
          :open-by-default="true"
        >
          <template #header>
            <h3 class="m-0 text-lg">{{ filter.formatted_name }}</h3>
          </template>
        </SearchSidebarFilter>
      </template>
    </div>
  </aside>
  <section class="normal-page__content">
    <div class="flex flex-col gap-3">
      <div class="iconified-input w-full">
        <SearchIcon aria-hidden="true" class="text-lg" />
        <input
          v-model="query"
          class="h-12"
          autocomplete="off"
          spellcheck="false"
          type="text"
          :placeholder="formatMessage(messages.searchPlaceholder)"
          @input="throttledSearch()"
        />
        <Button
          v-if="query"
          class="r-btn"
          @click="
            () => {
              query = ''
              updateSearchResults()
            }
          "
        >
          <XIcon />
        </Button>
      </div>
      <div class="flex flex-wrap items-center gap-2">
        <DropdownSelect
          v-slot="{ selected }"
          v-model="sortType"
          class="!w-auto flex-grow md:flex-grow-0"
          name="Sort by"
          :options="sortOptions.map((o) => o.value)"
          :display-name="
            (value?: SortOption) =>
              sortOptions.find((o) => o.value === value)?.label
          "
          @change="updateSearchResults()"
        >
          <span class="font-semibold text-primary"
            >{{ formatMessage(messages.sortBy) }}:
          </span>
          <span class="font-semibold text-secondary">{{ selected }}</span>
        </DropdownSelect>
        <DropdownSelect
          v-slot="{ selected }"
          v-model="perPage"
          name="Per page"
          :options="currentMaxResultsOptions"
          :default-value="perPage"
          class="!w-auto flex-grow md:flex-grow-0"
          @change="updateSearchResults()"
        >
          <span class="font-semibold text-primary"
            >{{ formatMessage(messages.view) }}:
          </span>
          <span class="font-semibold text-secondary">{{ selected }}</span>
        </DropdownSelect>
        <div class="lg:hidden">
          <ButtonStyled>
            <button @click="filtersMenuOpen = true">
              <FilterIcon />
              {{ formatMessage(messages.filters) }}...
            </button>
          </ButtonStyled>
        </div>
        <ButtonStyled circular>
          <button
            :v-tooltip="capitalizeString(resultsDisplayMode + ' view')"
            :aria-label="capitalizeString(resultsDisplayMode + ' view')"
            @click="cycleSearchDisplayMode()"
          >
            <GridIcon v-if="resultsDisplayMode === 'grid'" />
            <ImageIcon v-else-if="resultsDisplayMode === 'gallery'" />
            <ListIcon v-else />
          </button>
        </ButtonStyled>
        <Pagination
          :page="currentPage"
          :count="pageCount"
          class="mx-auto sm:ml-auto sm:mr-0"
          @switch-page="updateSearchResults"
        />
      </div>
      <!-- Selected filters display -->
      <SearchFilterControl
        v-model:selected-filters="currentFilters"
        :filters="filters"
        :provided-filters="providedFilters"
        :overridden-provided-filter-types="overriddenProvidedFilterTypes"
      />
      <LogoAnimated v-if="searchLoading" />
      <div v-else-if="searchResults.length === 0" class="no-results">
        <p>{{ formatMessage(messages.noResults) }}</p>
      </div>
      <div v-else class="search-results-container">
        <div
          id="search-results"
          class="project-list"
          :class="'display-mode--' + resultsDisplayMode"
          role="list"
          aria-label="Search results"
        >
          <ProjectCard
            v-for="result in searchResults"
            :key="result.id"
            :id="result.id"
            :display="resultsDisplayMode"
            type="plugin"
            :author="result.author"
            :name="result.name"
            :description="result.summary"
            :created-at="result.created_at"
            :updated-at="result.updated_at"
            :downloads="result.downloads.toString()"
            :stars="result.stars.toString()"
            :icon-url="result.icon_url"
            :categories="result.categories.slice(0, 3)"
            :search="true"
            :show-updated-date="sortType !== 'newest'"
            :show-created-date="true"
          />
        </div>
      </div>
      <div class="pagination-after">
        <Pagination
          :page="currentPage"
          :count="pageCount"
          class="justify-end"
          @switch-page="updateSearchResults"
        />
      </div>
    </div>
  </section>
</template>
<style lang="scss" scoped>
.normal-page__content {
  // Passthrough children as grid items on mobile
  display: contents;

  @media screen and (min-width: 1024px) {
    display: block;
  }
}

// Move the filters "sidebar" on mobile underneath the search card
.normal-page__sidebar {
  grid-row: 3;

  // Always show on desktop
  @media screen and (min-width: 1024px) {
    display: block;
  }
}

.filters-card {
  padding: var(--spacing-card-md);

  @media screen and (min-width: 1024px) {
    padding: var(--spacing-card-lg);
  }
}

.sidebar-menu {
  display: none;
}

.sidebar-menu_open {
  display: block;
}

.sidebar-menu-heading {
  margin: 1.5rem 0 0.5rem 0;
}

// EthicalAds
.content-wrapper {
  grid-row: 1;
}

.search-controls {
  display: flex;
  flex-direction: row;
  gap: var(--spacing-card-md);
  flex-wrap: wrap;
  padding: var(--spacing-card-md);
  grid-row: 2;

  .search-filter-container {
    display: flex;
    width: 100%;
    align-items: center;

    .sidebar-menu-close-button {
      max-height: none;
      // match height of the search field
      height: 40px;
      transition: box-shadow 0.1s ease-in-out;
      margin-right: var(--spacing-card-md);

      &.open {
        color: var(--color-button-text-active);
        background-color: var(--color-brand-highlight);
        box-shadow:
          inset 0 0 0 transparent,
          0 0 0 2px var(--color-brand);
      }
    }

    .iconified-input {
      flex: 1;

      input {
        width: 100%;
        margin: 0;
      }
    }
  }

  .sort-controls {
    width: 100%;
    display: flex;
    flex-direction: row;
    gap: var(--spacing-card-md);
    flex-wrap: wrap;
    align-items: center;

    .labeled-control {
      flex: 1;
      display: flex;
      flex-direction: column;
      align-items: center;
      flex-wrap: wrap;
      gap: 0.5rem;

      .labeled-control__label {
        white-space: nowrap;
      }
    }

    .square-button {
      margin-top: auto;
      // match height of search dropdowns
      height: 40px;
      width: 40px; // make it square!
    }
  }
}

.search-controls__sorting {
  min-width: 14rem;
}

.labeled-control__label,
.labeled-control__control {
  display: block;
}

.pagination-before {
  grid-row: 4;
}

.search-results-container {
  grid-row: 5;
}

.pagination-after {
  grid-row: 6;
}

.no-results {
  text-align: center;
  display: flow-root;
}

.loading-logo {
  margin: 2rem;
}

#search-results {
  min-height: 20vh;
}

@media screen and (min-width: 750px) {
  .search-controls {
    flex-wrap: nowrap;
    flex-direction: row;
  }

  .sort-controls {
    min-width: fit-content;
    max-width: fit-content;
    flex-wrap: nowrap;
  }

  .labeled-control {
    align-items: center;
    display: flex;
    flex-direction: column !important;
    flex-wrap: wrap;
    gap: 0.5rem;
    max-width: fit-content;
  }

  .labeled-control__label {
    flex-shrink: 0;
    margin-bottom: 0 !important;
  }
}

@media screen and (min-width: 860px) {
  .labeled-control {
    flex-wrap: nowrap !important;
    flex-direction: row !important;
  }
}

@media screen and (min-width: 1024px) {
  .sidebar-menu {
    display: block;
    margin-top: 0;
  }

  .sidebar-menu-close-button {
    display: none;
  }

  .labeled-control {
    flex-wrap: wrap !important;
    flex-direction: column !important;
  }
}

@media screen and (min-width: 1100px) {
  .labeled-control {
    flex-wrap: nowrap !important;
    flex-direction: row !important;
  }
}

.search-background {
  width: 100%;
  height: 20rem;
  background-image: url('https://minecraft.wiki/images/The_Garden_Awakens_Key_Art_No_Creaking.jpg?9968c');
  background-size: cover;
  background-position: center;
  pointer-events: none;
  mask-image: linear-gradient(to bottom, black, transparent);
  opacity: 0.25;
}

.stylized-toggle:checked::after {
  background: var(--color-accent-contrast) !important;
}
</style>
