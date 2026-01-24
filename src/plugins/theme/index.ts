import { useNativeTheme } from './native-theme.ts'
import { usePreferredThemes } from './preferred-theme.ts'
import { useThemeSettings } from './theme-settings.ts'
import { isDarkTheme } from './themes.ts'

export * from './themes.ts'

export default defineNuxtPlugin({
  name: 'theme',
  dependsOn: ['cosmetics'],
  setup() {
    const $nativeTheme = useNativeTheme()
    const $preferredThemes = usePreferredThemes()

    function getPreferredNativeTheme() {
      return $nativeTheme.value === 'light'
        ? $preferredThemes.light
        : $preferredThemes.dark
    }

    const $settings = useThemeSettings(() => getPreferredNativeTheme())

    useHead({
      script: [
        {
          innerHTML: `(function(){try{var c=document.cookie.match(/color-mode=([^;]+)/);if(c){var t=JSON.parse(decodeURIComponent(c[1]));if(t&&t.value){document.documentElement.classList.add(t.value+'-mode');return}}document.documentElement.classList.add('dark-mode')}catch(e){document.documentElement.classList.add('dark-mode')}})()`,
          tagPosition: 'head',
        },
      ],
    })

    function syncTheme() {
      $settings.active =
        $settings.preferred === 'system'
          ? getPreferredNativeTheme()
          : $settings.preferred

      if (import.meta.client) {
        const html = document.documentElement
        html.classList.forEach((cls) => {
          if (cls.endsWith('-mode')) {
            html.classList.remove(cls)
          }
        })
        html.classList.add(`${$settings.active}-mode`)
      }
    }

    if (import.meta.client) {
      watchEffect(() => {
        if ($settings.preferred) {
          syncTheme()
        }
      })
    }

    function cycle() {
      const nextTheme = isDarkTheme($settings.active)
        ? $preferredThemes.light
        : $preferredThemes.dark

      $settings.preferred = nextTheme

      return nextTheme
    }

    return {
      provide: {
        theme: reactive({
          ...toRefs($settings),
          preferences: $preferredThemes,
          native: $nativeTheme,
          cycle,
        }),
      },
    }
  },
})
