import { toValue, type Ref, type ComputedRef } from 'vue'
import type AllayIndex from '~/types/allayhub-index'
import { CATEGORIES, API_VERSIONS } from '~/types/allayhub'
import {
  searchPlugins,
  getAllPlugins,
  type OramaSearchFilters,
  type PluginDocument,
} from './orama-loader'

export type SortOption = 'downloads' | 'stars' | 'updated' | 'newest'

export interface SearchFilters {
  query?: string
  categories?: string[]
  apiMajor?: number
  license?: 'open-source' | 'closed-source'
}

export interface SearchOptions {
  sort?: SortOption
  limit?: number
  page?: number
  perPage?: number
}

/**
 * Get categories (static data)
 */
export function useCategories() {
  return { data: ref({ categories: CATEGORIES }) }
}

/**
 * Get Allay API versions (static data)
 */
export function useApiVersions() {
  return { data: ref({ versions: API_VERSIONS }) }
}

// Pre-load all plugin JSON files at build time using Vite's import.meta.glob
// From src/composables/ to AllayHubIndex/ requires ../../
// Plugins are stored in subdirectories: AllayHubIndex/{owner}/{name}.json
const pluginModules = import.meta.glob<AllayIndex.Plugin>(
  '../../AllayHubIndex/**/*.json',
  {
    eager: false,
    import: 'default',
  },
)

function processPluginData(data: AllayIndex.Plugin): AllayIndex.Plugin {
  const result: Record<string, unknown> = { ...data }
  for (const key of Object.keys(result)) {
    if (key.startsWith('!')) {
      const normalKey = key.slice(1)
      if (!(normalKey in result) || result[normalKey] === undefined) {
        result[normalKey] = result[key]
      }
      delete result[key]
    }
  }
  return result as unknown as AllayIndex.Plugin
}

/**
 * Find all full plugin IDs (owner/name) matching a plugin name.
 * Used for resolving dependency references which only contain the name.
 * Returns an array of matching IDs, or [input] if it already contains '/'.
 */
export function findPluginIdsByName(name: string): string[] {
  if (name.includes('/')) return [name]
  const suffix = `/${name}.json`
  const results: string[] = []
  for (const key of Object.keys(pluginModules)) {
    if (key.toLowerCase().endsWith(suffix.toLowerCase())) {
      const match = key.match(/AllayHubIndex\/(.+)\.json$/)
      if (match) results.push(match[1])
    }
  }
  return results.length > 0 ? results : [name]
}

/**
 * Load plugin details from pre-built JSON files
 * Works in both SSG and dev modes without server API
 */
export function usePlugin(
  pluginId: string | Ref<string> | ComputedRef<string>,
) {
  const id = toValue(pluginId)

  return useAsyncData<AllayIndex.Plugin>(
    `plugin-${id}`,
    async () => {
      const modulePath = `../../AllayHubIndex/${id}.json`
      const loader = pluginModules[modulePath]
      if (!loader) {
        throw new Error(`Plugin not found: ${id}`)
      }
      const rawData = await loader()
      return processPluginData(rawData)
    },
    {
      watch:
        typeof pluginId === 'string' ? undefined : [pluginId as Ref<string>],
    },
  )
}

/**
 * Get a specific version from plugin data
 */
export function getPluginVersion(
  plugin: AllayIndex.Plugin | null | undefined,
  versionId: string,
): AllayIndex.RawVersion | undefined {
  if (!plugin) return undefined
  return plugin.versions.find((v) => v.version === versionId)
}

/**
 * Get the latest version from plugin data
 */
export function getLatestVersion(
  plugin: AllayIndex.Plugin | null | undefined,
  releaseType?: 'release' | 'beta',
): AllayIndex.RawVersion | undefined {
  if (!plugin || plugin.versions.length === 0) return undefined
  const versions = releaseType
    ? plugin.versions.filter((v) => {
        const isPrerelease = v.prerelease
        if (releaseType === 'release') return !isPrerelease
        return isPrerelease
      })
    : plugin.versions
  return versions[0]
}

/**
 * Get primary download file from version
 */
export function getPrimaryFile(
  version: AllayIndex.Version | AllayIndex.RawVersion | null | undefined,
): AllayIndex.VersionFile | undefined {
  if (!version) return undefined
  return version.files.find((f) => f.primary) || version.files[0]
}

/**
 * Format file size for display
 */
export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

/**
 * Convert unix timestamp (seconds) to ISO string
 */
function toISOString(seconds: number): string {
  return new Date(seconds * 1000).toISOString()
}

/**
 * Get total pages for pagination
 */
export function getTotalPages(totalItems: number, perPage: number): number {
  return Math.ceil(totalItems / perPage)
}

function toPluginSummary(doc: PluginDocument): AllayIndex.PluginSummary {
  return {
    id: doc.id,
    name: doc.display_name,
    summary: doc.summary,
    author: doc.author,
    categories: doc.categories,
    api_version: doc.api_version,
    license: doc.license,
    downloads: doc.downloads,
    stars: doc.stars,
    created_at: toISOString(doc.created_at),
    updated_at: toISOString(doc.updated_at),
    icon_url: doc.icon_url || undefined,
    gallery_image: doc.gallery_image || undefined,
  }
}

/**
 * Plugin search composable
 */
export function usePluginSearch() {
  const isSearching = ref(false)
  const searchError = ref<Error | null>(null)
  const isIndexLoaded = ref(false)

  async function search(
    filters: SearchFilters = {},
    options: SearchOptions = {},
  ): Promise<AllayIndex.PluginSummary[]> {
    const { query, categories, apiMajor, license } = filters
    const { sort = 'downloads', limit, page, perPage = 20 } = options

    isSearching.value = true
    searchError.value = null

    try {
      const oramaFilters: OramaSearchFilters = {}
      if (categories?.length) {
        oramaFilters.categories = categories
      }
      if (license) {
        oramaFilters.license = license
      }
      if (apiMajor !== undefined) {
        oramaFilters.apiMajor = apiMajor
      }

      const searchLimit = limit ?? (page ? perPage : 100)
      const searchOffset = page ? (page - 1) * perPage : 0

      const results = await searchPlugins({
        term: query?.trim() || '',
        filters: oramaFilters,
        sortBy: sort,
        limit: searchLimit,
        offset: searchOffset,
      })

      isIndexLoaded.value = true
      return results.hits.map(toPluginSummary)
    } catch (e) {
      searchError.value = e as Error
      console.error('Search error:', e)
      return []
    } finally {
      isSearching.value = false
    }
  }

  async function getTotalCount(filters: SearchFilters = {}): Promise<number> {
    const { query, categories, apiMajor, license } = filters

    const oramaFilters: OramaSearchFilters = {}
    if (categories?.length) {
      oramaFilters.categories = categories
    }
    if (license) {
      oramaFilters.license = license
    }
    if (apiMajor !== undefined) {
      oramaFilters.apiMajor = apiMajor
    }

    const results = await searchPlugins({
      term: query?.trim() || '',
      filters: oramaFilters,
      limit: 10000,
    })

    return results.count
  }

  async function getByCategory(
    categoryId: string,
    options: SearchOptions = {},
  ): Promise<AllayIndex.PluginSummary[]> {
    return search(
      { categories: [categoryId] },
      { sort: 'downloads', ...options },
    )
  }

  async function getRecentlyUpdated(
    limit = 10,
  ): Promise<AllayIndex.PluginSummary[]> {
    return search({}, { sort: 'updated', limit })
  }

  async function getPopular(limit = 10): Promise<AllayIndex.PluginSummary[]> {
    return search({}, { sort: 'downloads', limit })
  }

  async function getFeatured(limit = 10): Promise<AllayIndex.PluginSummary[]> {
    return search({}, { sort: 'stars', limit })
  }

  async function preloadIndex(): Promise<void> {
    try {
      await getAllPlugins({}, 'downloads', 1)
      isIndexLoaded.value = true
    } catch (e) {
      console.warn('Failed to preload search index:', e)
    }
  }

  return {
    isSearching,
    searchError,
    isIndexLoaded,
    search,
    getTotalCount,
    getByCategory,
    getRecentlyUpdated,
    getPopular,
    getFeatured,
    preloadIndex,
  }
}
