export interface Category {
  id: string
  name: string
  description: string
}

export interface ApiVersion {
  version: string
  release_date: string
}

export interface ServerVersion {
  version: string
  api_version: string
  release_date: string
}

export interface IndexMeta {
  updated_at: string
  index_version: string
  generator: string
}

export interface Loader {
  id: string
  name: string
  icon?: string
}

export const CATEGORIES: Category[] = [
  { id: 'adventure', name: 'Adventure', description: 'Adventure and exploration plugins' },
  { id: 'cursed', name: 'Cursed', description: 'Cursed and challenge plugins' },
  { id: 'decoration', name: 'Decoration', description: 'Decoration and building plugins' },
  { id: 'economy', name: 'Economy', description: 'Economy and trading plugins' },
  { id: 'equipment', name: 'Equipment', description: 'Equipment and gear plugins' },
  { id: 'food', name: 'Food', description: 'Food and farming plugins' },
  { id: 'game-mechanics', name: 'Game Mechanics', description: 'Game mechanics modification plugins' },
  { id: 'library', name: 'Library', description: 'API libraries for developers' },
  { id: 'magic', name: 'Magic', description: 'Magic and spells plugins' },
  { id: 'management', name: 'Management', description: 'Server management plugins' },
  { id: 'minigame', name: 'Minigame', description: 'Minigame plugins' },
  { id: 'mobs', name: 'Mobs', description: 'Mob related plugins' },
  { id: 'optimization', name: 'Optimization', description: 'Performance optimization plugins' },
  { id: 'social', name: 'Social', description: 'Social and communication plugins' },
  { id: 'storage', name: 'Storage', description: 'Storage and inventory plugins' },
  { id: 'technology', name: 'Technology', description: 'Technology and automation plugins' },
  { id: 'transportation', name: 'Transportation', description: 'Transportation plugins' },
  { id: 'utility', name: 'Utility', description: 'General utility plugins' },
  { id: 'world-generation', name: 'World Generation', description: 'World generation plugins' },
]

export const API_VERSIONS: ApiVersion[] = [
  { version: '1.0.0', release_date: '2026-01-01' },
]

export const LATEST_API_VERSION = '1.0.0'

export const SERVER_VERSIONS: ServerVersion[] = [
  { version: '1.0.0', api_version: '1.0.0', release_date: '2026-01-01' },
]

export const INDEX_META: IndexMeta = {
  updated_at: '2026-01-20T15:18:17Z',
  index_version: '1.0',
  generator: 'allayindexer',
}

export const LOADERS: Loader[] = [
  { id: 'plugin', name: 'Plugin', icon: 'plugin' },
]

export type LoaderId = 'plugin'

export type CategoryId =
  | 'adventure'
  | 'cursed'
  | 'decoration'
  | 'economy'
  | 'equipment'
  | 'food'
  | 'game-mechanics'
  | 'library'
  | 'magic'
  | 'management'
  | 'minigame'
  | 'mobs'
  | 'optimization'
  | 'social'
  | 'storage'
  | 'technology'
  | 'transportation'
  | 'utility'
  | 'world-generation'

export function getCategoryById(id: string): Category | undefined {
  return CATEGORIES.find((c) => c.id === id)
}

export function getCategoryName(id: string): string {
  return getCategoryById(id)?.name ?? id
}

export function getCategoryIds(): string[] {
  return CATEGORIES.map((c) => c.id)
}

export function getLoaderById(id: string): Loader | undefined {
  return LOADERS.find((l) => l.id === id)
}

export function getLoaderName(id: string): string {
  return getLoaderById(id)?.name ?? id
}

export function getLoaderIcon(id: string): string | undefined {
  return getLoaderById(id)?.icon
}
