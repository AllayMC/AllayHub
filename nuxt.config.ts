import { LOCALES } from '@modrinth/ui/src/composables/i18n.ts'
import serverSidedVue from '@vitejs/plugin-vue'
import { defineNuxtConfig } from 'nuxt/config'
import svgLoader from 'vite-svg-loader'

const favicons = {
  '(prefers-color-scheme:no-preference)': '/favicon.ico',
  '(prefers-color-scheme:light)': '/favicon.ico',
  '(prefers-color-scheme:dark)': '/favicon.ico',
}

export default defineNuxtConfig({
  srcDir: 'src/',
  app: {
    head: {
      htmlAttrs: {
        lang: 'en',
      },
      title: 'AllayHub',
      link: [
        ...Object.entries(favicons).map(([media, href]): object => {
          return { rel: 'icon', type: 'image/x-icon', href, media }
        }),
        ...Object.entries(favicons).map(([media, href]): object => {
          return {
            rel: 'apple-touch-icon',
            type: 'image/x-icon',
            href,
            media,
            sizes: '64x64',
          }
        }),
        {
          rel: 'search',
          type: 'application/opensearchdescription+xml',
          href: '/opensearch.xml',
          title: 'AllayHub plugins',
        },
      ],
    },
  },
  vite: {
    css: {
      preprocessorOptions: {
        scss: {
          // TODO: dont forget about this
          silenceDeprecations: ['import'],
        },
      },
    },
    ssr: {
      // https://github.com/Akryum/floating-vue/issues/809#issuecomment-1002996240
      noExternal: ['v-tooltip', 'xss', 'intl-messageformat'],
    },
    define: {
      global: {},
    },
    esbuild: {
      define: {
        global: 'globalThis',
      },
    },
    cacheDir: '../../node_modules/.vite/apps/knossos',
    resolve: {
      dedupe: ['vue'],
    },
    plugins: [
      svgLoader({
        svgoConfig: {
          plugins: [
            {
              name: 'preset-default',
              params: {
                overrides: {
                  removeViewBox: false,
                },
              },
            },
          ],
        },
      }),
    ],
    build: {},
    server: {
      fs: {
        // Allow access to test directory for mock data
        allow: ['..'],
      },
    },
  },
  // SSG mode configuration
  nitro: {
    // Static site generation preset
    preset: 'static',
    // Prerender configuration
    prerender: {
      // Crawl all links from starting routes
      crawlLinks: true,
      // Starting routes for crawling
      routes: ['/'],
      // Ignore routes that intentionally throw 404
      ignore: ['/discover'],
      // Ignore errors for dynamic routes that can't be pre-rendered
      failOnError: true,
    },
    rollupConfig: {
      plugins: [serverSidedVue()],
    },
    hooks: {
      'prerender:generate': (route) => {
        if (route.error) {
          console.error('Prerender error for', route.route)
          console.error('Error:', route.error)
          if (route.error.stack) {
            console.error('Stack:', route.error.stack)
          }
        }
      },
    },
  },
  runtimeConfig: {
    public: {
      siteUrl: getDomain(),
      production: isProduction(),
      featureFlagOverrides: getFeatureFlagOverrides(),

      owner: process.env.VERCEL_GIT_REPO_OWNER || 'AllayMC',
      slug: process.env.VERCEL_GIT_REPO_SLUG || 'AllayHub',
      branch:
        process.env.VERCEL_GIT_COMMIT_REF ||
        process.env.CF_PAGES_BRANCH ||
        // @ts-ignore
        globalThis.CF_PAGES_BRANCH ||
        'main',
      hash:
        process.env.VERCEL_GIT_COMMIT_SHA ||
        process.env.CF_PAGES_COMMIT_SHA ||
        // @ts-ignore
        globalThis.CF_PAGES_COMMIT_SHA ||
        'unknown',
    },
  },
  typescript: {
    shim: false,
    strict: true,
    typeCheck: false,
    tsConfig: {
      compilerOptions: {
        moduleResolution: 'bundler',
        allowImportingTsExtensions: true,
      },
    },
  },
  modules: ['@nuxtjs/i18n', '@pinia/nuxt', 'floating-vue/nuxt'],
  // @ts-ignore floating-vue module config
  floatingVue: {
    themes: {
      'ribbit-popout': {
        $extend: 'dropdown',
        placement: 'bottom-end',
        instantMove: true,
        distance: 8,
      },
      'dismissable-prompt': {
        $extend: 'dropdown',
        placement: 'bottom-start',
      },
    },
  },
  i18n: {
    defaultLocale: 'en-US',
    lazy: true,
    langDir: '.',
    locales: LOCALES.map((locale) => ({
      ...locale,
      file: 'locale-loader.ts',
    })),
    strategy: 'no_prefix',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'locale',
      fallbackLocale: 'en-US',
    },
    vueI18n: './i18n.config.ts',
    bundle: {
      optimizeTranslationDirective: false,
    },
  },
  devtools: {
    enabled: true,
  },
  css: ['~/assets/styles/tailwind.css'],
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
  routeRules: {
    '/**': {
      headers: {
        'Accept-CH': 'Sec-CH-Prefers-Color-Scheme',
        'Critical-CH': 'Sec-CH-Prefers-Color-Scheme',
      },
    },
  },
  compatibilityDate: '2025-01-01',
  telemetry: false,
})

function isProduction() {
  return process.env.NODE_ENV === 'production'
}

function getFeatureFlagOverrides() {
  return JSON.parse(process.env.FLAG_OVERRIDES ?? '{}')
}

function getDomain() {
  if (process.env.NODE_ENV === 'production') {
    // @ts-ignore
    if (process.env.CF_PAGES_URL || globalThis.CF_PAGES_URL) {
      // @ts-ignore
      return process.env.CF_PAGES_URL ?? globalThis.CF_PAGES_URL
    } else if (process.env.HEROKU_APP_NAME) {
      return `https://${process.env.HEROKU_APP_NAME}.herokuapp.com`
    } else if (process.env.VERCEL_URL) {
      return `https://${process.env.VERCEL_URL}`
    } else {
      return 'https://allayhub.org'
    }
  } else {
    const port = process.env.PORT || 3000
    return `http://localhost:${port}`
  }
}
