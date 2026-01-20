import semver from 'semver'

/**
 * Check if a plugin version is outdated (not compatible with latest API)
 * For 0.x.x API versions, always return false (compatible)
 */
export function isVersionOutdated(
  apiVersion: string | undefined,
  latestApiVersion: string,
): boolean {
  if (!apiVersion) return false

  const latest = semver.parse(latestApiVersion)
  if (!latest) return false

  // For 0.x.x versions, skip checking - always compatible
  if (latest.major === 0) return false

  const version = semver.parse(apiVersion)
  if (!version) return false

  // If plugin API version > latest, it's outdated (built for future API)
  // If plugin API version <= latest, it's compatible
  return semver.gt(version, latest)
}

/**
 * Check if plugin is outdated based on its API version
 */
export function isPluginOutdated(
  apiVersion: string | undefined,
  latestApiVersion: string,
): boolean {
  return isVersionOutdated(apiVersion, latestApiVersion)
}
