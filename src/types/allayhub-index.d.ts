/**
 * AllayHub Index Data Types
 * Based on docs/INDEX_SCHEMA.md
 */

declare namespace AllayIndex {
  interface PluginSummary {
    // Basic info
    id: string // Unique identifier (URL-friendly), e.g. "my-plugin"
    name: string // Display name (tokenized, e.g. "allay essentials")
    summary: string // Short description (< 200 chars)

    // Author (first/primary author only for list display)
    author: string // e.g. "Cdm2883"

    // Categories
    categories: string[] // Category IDs, e.g. ["utility", "economy"]

    // Allay compatibility (for filtering)
    api_version: string // API version requirement, e.g. ">=0.16.0"

    // License
    license: 'open-source' | 'closed-source' // License type

    // Media
    icon_url?: string // Icon URL
    gallery_image?: string // Main image for gallery mode

    // Statistics
    downloads: number // Total downloads
    stars: number // GitHub Stars

    // Timestamps
    created_at: string // ISO 8601
    updated_at: string // ISO 8601
  }

  interface Plugin {
    id: string
    name: string
    source: string // GitHub repository URL (required)
    summary: string
    description: string // Markdown (README)

    authors: Author[]
    categories: string[]

    license: License // SPDX license info; ARR if no license file

    links?: PluginLinks

    icon_url?: string
    gallery?: GalleryImage[] // Gallery images for detail page

    downloads: number
    stars: number

    created_at: number // Unix timestamp
    updated_at: number // Unix timestamp

    versions: RawVersion[]

    api_version: string // Current API version from main branch
    server_version?: string // Server API version if used
    dependencies?: Dependency[] // Dependencies from main branch
  }

  interface Author {
    name: string
    url?: string // GitHub profile
    avatar_url?: string
  }

  interface License {
    id: string // SPDX ID, e.g. "MIT"
    name: string
    url?: string
  }

  interface PluginLinks {
    homepage?: string
    wiki?: string
    discord?: string
  }

  interface DonationLink {
    platform: string // "github", "patreon", "ko-fi", etc.
    url: string
  }

  interface GalleryImage {
    url: string
    title?: string
    description?: string
    created?: string // ISO 8601 date string
  }

  interface Dependency {
    plugin_id: string
    version_range?: string
    dependency_type: 'required' | 'optional'
  }

  interface IndexMeta {
    updated_at: string // ISO 8601, last index update time
    index_version: string // Schema version, e.g. "1.0"
    generator?: string // Indexer version, e.g. "allayhub-indexer/0.1.0"
  }

  interface CategoriesFile {
    categories: Category[]
  }

  interface Category {
    id: string // Unique identifier
    name: string // Display name
    icon?: string // Icon
    description?: string
  }

  interface ApiVersionsFile {
    versions: ApiVersion[]
  }

  interface ApiVersion {
    version: string // API version number, e.g. "0.16.0"
    release_date?: string // ISO 8601
    changelog_url?: string // Changelog link
  }

  /** Project data for template display */
  interface ProjectView {
    id: string
    slug: string
    project_type: string
    title: string
    description: string
    body: string
    icon_url?: string
    status: string
    license: {
      id: string
      name: string
      url: string | null
    }
    downloads: number
    followers: number
    categories: string[]
    game_versions: string[]
    loaders: string[]
    gallery: GalleryImage[]
    versions: Version[]
    published: string
    updated: string
    approved: string
    queued: string
    color: number
    // Links
    issues_url: string
    source_url: string
    wiki_url: string
    discord_url: string
    donation_url: DonationLink
    // Compatibility info (from main branch)
    api_version: string
    server_version?: string
    dependencies?: Dependency[]
  }

  /** Raw version data from Rust indexer JSON */
  interface RawVersion {
    version: string
    name: string
    prerelease: boolean
    changelog: string
    files: VersionFile[]
    downloads: number
    published_at: number // Unix timestamp
  }

  /** Version data for template display (transformed from RawVersion) */
  interface Version {
    id: string
    project_id: string
    name: string
    version: string
    changelog?: string
    published_at: string // ISO date string
    downloads: number
    prerelease: boolean
    author_id: string
    files: VersionFile[]
  }

  /** Version file for template display */
  interface VersionFile {
    url: string
    filename: string
    size: number
    primary: boolean
  }

  /** Member data for template display */
  interface MemberView {
    id: string
    team_id: string
    user: {
      id: string
      username: string
      avatar_url: string
    }
    role: string
    is_owner: boolean
    accepted: boolean
    permissions: number
    payouts_split: number
  }
}

export = AllayIndex
export as namespace AllayIndex
