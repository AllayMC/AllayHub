import type { DarkTheme } from './theme/index.ts'

export type DisplayMode = 'list' | 'gallery' | 'grid'

export type DisplayLocation = 'plugin'

export interface Cosmetics {
  rightSearchLayout: boolean
  leftContentLayout: boolean
  advancedRendering: boolean
  notUsingBlockers: boolean
  preferredDarkTheme: DarkTheme
  searchDisplayMode: Record<DisplayLocation, DisplayMode>
  hideStagingBanner: boolean
}

export default defineNuxtPlugin({
  name: 'cosmetics',
  setup() {
    const cosmetics = useCookie<Cosmetics>('cosmetics', {
      maxAge: 60 * 60 * 24 * 365 * 10,
      sameSite: 'lax',
      secure: true,
      httpOnly: false,
      path: '/',
      default: () => ({
        rightSearchLayout: false,
        leftContentLayout: false,
        advancedRendering: true,
        notUsingBlockers: false,
        preferredDarkTheme: 'dark',
        searchDisplayMode: {
          plugin: 'list',
        },
        hideStagingBanner: false,
      }),
    })

    return { provide: { cosmetics } }
  },
})
