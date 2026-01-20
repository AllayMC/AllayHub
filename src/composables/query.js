export function getArrayOrString(value) {
  if (value === undefined || value === null) {
    return null
  }
  return Array.isArray(value) ? value : [value]
}
