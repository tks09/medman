<script setup>
import { ref } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'

const authStore = useAuthStore()
const router = useRouter()
const { t } = useI18n()

const username = ref('')
const password = ref('')
const error = ref(null)
const isLoading = ref(false)

const handleLogin = async () => {
  try {
    error.value = null
    isLoading.value = true

    await authStore.login(username.value, password.value)

    router.push('/dashboard')
  } catch (err) {
    error.value = err.error || err.message || t('login.error')
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="max-w-md mx-auto mt-12">
    <div class="bg-navy-800 p-8 rounded-lg shadow-lg border border-navy-700">
      <h2 class="text-2xl font-bold text-primary-400 text-center mb-6">{{ t('login.title') }}</h2>

      <form @submit.prevent="handleLogin" class="space-y-4">
        <div>
          <label for="username" class="block text-sm font-medium text-gray-300 mb-1">{{ t('login.username') }}</label>
          <input
            id="username"
            v-model="username"
            type="text"
            required
            class="w-full px-3 py-2 bg-navy-900 border border-navy-600 text-gray-100 placeholder-gray-500 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-all"
          />
        </div>

        <div>
          <label for="password" class="block text-sm font-medium text-gray-300 mb-1">{{ t('login.password') }}</label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            class="w-full px-3 py-2 bg-navy-900 border border-navy-600 text-gray-100 placeholder-gray-500 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-all"
          />
        </div>

        <div v-if="error" class="bg-red-950 border border-red-700 text-red-300 px-4 py-3 rounded text-sm">
          {{ error }}
        </div>

        <button
          type="submit"
          :disabled="isLoading"
          class="w-full bg-primary-600 text-white py-3 px-4 rounded-lg hover:bg-primary-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-lg hover:shadow-xl"
        >
          <span v-if="isLoading">{{ t('login.loggingIn') }}</span>
          <span v-else>{{ t('login.login') }}</span>
        </button>
      </form>

      <div class="mt-4 text-center">
        <p class="text-sm text-gray-400">
          {{ t('login.dontHaveAccount') }}
          <RouterLink to="/register" class="text-primary-400 hover:text-primary-300 hover:underline transition-colors">{{ t('login.register') }}</RouterLink>
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Component-specific styles */
</style>
