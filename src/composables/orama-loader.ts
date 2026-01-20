/**
 * Orama Search Index Loader
 *
 * Lazy-loads the pre-built Orama index for client-side search.
 * Uses singleton pattern to prevent multiple index loads.
 */

import { search as oramaSearch, type Orama } from '@orama/orama'
import { restore } from '@orama/plugin-data-persistence'

/**
 * Document returned from search
 * Matches structure built in build-search.ts
 */
export interface PluginDocument {
  name: string
  owner: string
  categories: string[]
  license: 'open-source' | 'closed-source'
  api_major: number
  downloads: number
  stars: number
  created_at: number
  updated_at: number
  id: string
  display_name: string
  author: string
  summary: string
  icon_url: string
  gallery_image: string
  api_version: string
}

export type PluginSearchDB = Orama<{
  name: 'string'
  owner: 'string'
  categories: 'enum[]'
  license: 'enum'
  api_major: 'number'
  downloads: 'number'
  stars: 'number'
  created_at: 'number'
  updated_at: 'number'
}>

export interface OramaSearchFilters {
  categories?: string[]
  license?: 'open-source' | 'closed-source'
  apiMajor?: number
}

export interface OramaSearchOptions {
  term?: string
  filters?: OramaSearchFilters
  limit?: number
  offset?: number
  sortBy?: 'downloads' | 'stars' | 'updated' | 'newest'
}

export interface SearchResult {
  hits: PluginDocument[]
  count: number
  elapsed: number
}

// Singleton cache
const CACHE_KEY = '__orama_cache__'

interface OramaCache {
  db: PluginSearchDB | null
  loading: Promise<PluginSearchDB> | null
}

function getCache(): OramaCache {
  if (!(globalThis as any)[CACHE_KEY]) {
    ;(globalThis as any)[CACHE_KEY] = { db: null, loading: null }
  }
  return (globalThis as any)[CACHE_KEY]
}

/**
 * Load the Orama search index
 */
export async function loadOramaIndex(): Promise<PluginSearchDB> {
  const cache = getCache()

  if (cache.db) return cache.db
  if (cache.loading) return cache.loading

  cache.loading = (async () => {
    try {
      const response = await fetch('/orama-index.bin')
      const data = await response.arrayBuffer()
      const db = await restore('seqproto', data, undefined, {
        sort: { enabled: true },
      })
      cache.db = db as PluginSearchDB
      return cache.db
    } catch (e) {
      console.error('Failed to load Orama index:', e)
      throw e
    } finally {
      cache.loading = null
    }
  })()

  return cache.loading
}

/**
 * Search plugins
 */
export async function searchPlugins(
  options: OramaSearchOptions = {},
): Promise<SearchResult> {
  const {
    term = '',
    filters = {},
    limit = 20,
    offset = 0,
    sortBy = 'downloads',
  } = options

  const db = await loadOramaIndex()

  const where: Record<string, any> = {}
  if (filters.categories?.length) {
    where.categories = { containsAny: filters.categories }
  }
  if (filters.license) {
    where.license = { eq: filters.license }
  }
  if (filters.apiMajor !== undefined) {
    where.api_major = { lte: filters.apiMajor }
  }

  // Build sort options
  const sortByOptions = (() => {
    switch (sortBy) {
      case 'downloads':
        return { property: 'downloads', order: 'DESC' as const }
      case 'stars':
        return { property: 'stars', order: 'DESC' as const }
      case 'updated':
        return { property: 'updated_at', order: 'DESC' as const }
      case 'newest':
        return { property: 'created_at', order: 'DESC' as const }
    }
  })()

  const whereClause = Object.keys(where).length > 0 ? where : undefined

  const results = await oramaSearch(db, {
    term,
    properties: ['name', 'owner'],
    boost: { name: 2, owner: 1 },
    where: whereClause,
    limit,
    offset,
    sortBy: sortByOptions,
    tolerance: 1,
  })

  return {
    hits: results.hits.map((hit) => hit.document as PluginDocument),
    count: results.count,
    elapsed: results.elapsed?.raw ?? 0,
  }
}

/**
 * Get all plugins (list with optional filters)
 */
export async function getAllPlugins(
  filters: OramaSearchFilters = {},
  sortBy: OramaSearchOptions['sortBy'] = 'downloads',
  limit = 1000,
): Promise<SearchResult> {
  return searchPlugins({ term: '', filters, sortBy, limit })
}

/**
 * Reset cached index
 */
export function resetOramaLoader() {
  const cache = getCache()
  cache.db = null
  cache.loading = null
}
