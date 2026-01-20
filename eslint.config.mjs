import config from '@modrinth/tooling-config/eslint/nuxt.mjs'
export default config.append([
  {
    rules: {
      '@typescript-eslint/no-explicit-any': 'error',
      'import/no-unresolved': 'off',
      'no-undef': 'off',
    },
  },
])
