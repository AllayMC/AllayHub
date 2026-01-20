<template>
  <NuxtLayout>
    <div class="error-container">
      <div class="error-box">
        <h1 class="error-title">{{ errorTitle }}</h1>
        <p class="error-message">{{ errorMessage }}</p>
        <NuxtLink to="/" class="error-link">Back to Home</NuxtLink>
      </div>
    </div>
  </NuxtLayout>
</template>

<script setup lang="ts">
const props = defineProps({
  error: {
    type: Object,
    default() {
      return {
        statusCode: 500,
        message: 'Unknown error',
      }
    },
  },
})

const errorTitle = computed(() => {
  switch (props.error.statusCode) {
    case 404:
      return 'Page Not Found'
    case 500:
      return 'Server Error'
    default:
      return `Error ${props.error.statusCode}`
  }
})

const errorMessage = computed(() => {
  switch (props.error.statusCode) {
    case 404:
      return "The page you're looking for doesn't exist."
    default:
      return props.error.message || 'Something went wrong.'
  }
})
</script>

<style scoped>
.error-container {
  min-height: 60vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
}

.error-box {
  text-align: center;
  max-width: 400px;
}

.error-title {
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 1rem;
}

.error-message {
  color: var(--color-secondary, #666);
  margin-bottom: 2rem;
}

.error-link {
  display: inline-block;
  padding: 0.75rem 1.5rem;
  background: var(--color-brand, #1bd96a);
  color: white;
  border-radius: 0.5rem;
  text-decoration: none;
  font-weight: 500;
}

.error-link:hover {
  opacity: 0.9;
}
</style>
