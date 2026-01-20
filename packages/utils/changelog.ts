import dayjs from 'dayjs'

export type Product = 'web' | 'hosting' | 'api' | 'app'

export type VersionEntry = {
  date: dayjs.Dayjs
  product: Product
  version?: string
  body: string
}

const VERSIONS: VersionEntry[] = []

export function getChangelog() {
  return VERSIONS
}
