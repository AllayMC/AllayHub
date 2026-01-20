export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.provide('formatNumber', formatNumber)
  nuxtApp.provide('capitalizeString', capitalizeString)
  nuxtApp.provide('formatMoney', formatMoney)
  nuxtApp.provide('orElse', (first, otherwise) => first ?? otherwise)

  /*
    Only use on the complete list of versions for a project, partial lists will generate
    the wrong version slugs
  */
  nuxtApp.provide('computeVersions', (versions, members) => {
    const visitedVersions = []
    const returnVersions = []

    const authorMembers = {}

    for (const version of versions.sort(
      (a, b) => nuxtApp.$dayjs(a.published_at) - nuxtApp.$dayjs(b.published_at),
    )) {
      if (visitedVersions.includes(version.version)) {
        visitedVersions.push(version.version)
        version.displayUrlEnding = version.id
      } else {
        visitedVersions.push(version.version)
        version.displayUrlEnding = version.version
      }
      version.primaryFile =
        version.files.find((file) => file.primary) ?? version.files[0]

      if (!version.primaryFile) {
        version.primaryFile = {
          url: '#',
          filename: 'unknown',
          primary: false,
          size: 0,
        }
      }

      version.author = authorMembers[version.author_id]
      if (!version.author) {
        version.author = members.find((x) => x.user.id === version.author_id)
        authorMembers[version.author_id] = version.author
      }

      returnVersions.push(version)
    }

    return returnVersions
      .reverse()
      .map((version, index) => {
        const nextVersion = returnVersions[index + 1]
        if (
          nextVersion &&
          version.changelog &&
          nextVersion.changelog === version.changelog
        ) {
          return { duplicate: true, ...version }
        } else {
          return { duplicate: false, ...version }
        }
      })
      .sort(
        (a, b) =>
          nuxtApp.$dayjs(b.published_at) - nuxtApp.$dayjs(a.published_at),
      )
  })
  // AllayHub only has plugins, so simplified version
  nuxtApp.provide('getProjectTypeForDisplay', (type, _categories) => {
    // AllayHub only supports plugins
    return type === 'plugin' ? 'plugin' : type
  })
  nuxtApp.provide('getProjectTypeForUrl', () => 'plugin')
  nuxtApp.provide('cycleValue', cycleValue)
  // AllayHub categories are simpler - just return empty array
  // This is a placeholder since AllayHub uses its own category system
  nuxtApp.provide('sortedCategories', () => [])
})
export const formatNumber = (number, abbreviate = true) => {
  const x = +number
  if (x >= 1000000 && abbreviate) {
    return (x / 1000000).toFixed(2).toString() + 'M'
  } else if (x >= 10000 && abbreviate) {
    return (x / 1000).toFixed(1).toString() + 'k'
  } else {
    return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')
  }
}

export const formatMoney = (number, abbreviate = false) => {
  number = Math.floor(number * 100) / 100
  const x = +number
  if (x >= 1000000 && abbreviate) {
    return '$' + (x / 1000000).toFixed(2).toString() + 'M'
  } else if (x >= 10000 && abbreviate) {
    return '$' + (x / 1000).toFixed(2).toString() + 'k'
  } else {
    return (
      '$' +
      x
        .toFixed(2)
        .toString()
        .replace(/\B(?=(\d{3})+(?!\d))/g, ',')
    )
  }
}

export const capitalizeString = (name) => {
  return name ? name.charAt(0).toUpperCase() + name.slice(1) : name
}

export const cycleValue = (value, values) => {
  const index = values.indexOf(value) + 1
  return values[index % values.length]
}
