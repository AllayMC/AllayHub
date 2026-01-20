export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.hook('app:error', (error: any) => {
    console.error('=== APP ERROR ===')
    console.error('Message:', error?.message)
    console.error('Stack:', error?.stack)
    console.error('Full error:', error)
  })

  nuxtApp.hook('vue:error', (error: any, instance: any, info: string) => {
    console.error('=== VUE ERROR ===')
    console.error('Info:', info)
    console.error('Message:', error?.message)
    console.error('Stack:', error?.stack)
    console.error('Component:', instance?.$options?.name || instance?.type?.name)
  })
})
