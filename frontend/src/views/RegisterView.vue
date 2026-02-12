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
const confirmPassword = ref('')
const error = ref(null)
const isLoading = ref(false)

const handleRegister = async () => {
  if (password.value !== confirmPassword.value) {
    error.value = t('register.passwordsDoNotMatch')
    return
  }

  try {
    error.value = null
    isLoading.value = true

    await authStore.register(username.value, password.value)

    router.push('/dashboard')
  } catch (err) {
    error.value = err.error || err.message || t('register.error')
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="max-w-md mx-auto mt-12">
    <div class="bg-white p-8 rounded-lg shadow-md">
      <h2 class="text-2xl font-bold text-primary-600 text-center mb-6">{{ t('register.title') }}</h2>

      <form @submit.prevent="handleRegister" class="space-y-4">
        <div>
          <label for="username" class="block text-sm font-medium text-gray-700 mb-1">{{ t('register.username') }}</label>
          <input
            id="username"
            v-model="username"
            type="text"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
          />
        </div>

        <div>
          <label for="password" class="block text-sm font-medium text-gray-700 mb-1">{{ t('register.password') }}</label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            minlength="6"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
          />
        </div>

        <div>
          <label for="confirmPassword" class="block text-sm font-medium text-gray-700 mb-1">{{ t('register.confirmPassword') }}</label>
          <input
            id="confirmPassword"
            v-model="confirmPassword"
            type="password"
            required
            minlength="6"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
          />
        </div>

        <div v-if="error" class="text-red-600 text-sm">
          {{ error }}
        </div>

        <button
          type="submit"
          :disabled="isLoading"
          class="w-full bg-green-600 text-white py-2 px-4 rounded-md hover:bg-green-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <span v-if="isLoading">{{ t('register.registering') }}</span>
          <span v-else>{{ t('register.register') }}</span>
        </button>
      </form>

      <div class="mt-4 text-center">
        <p class="text-sm text-gray-600">
          {{ t('register.alreadyHaveAccount') }}
          <RouterLink to="/login" class="text-primary-600 hover:underline">{{ t('register.login') }}</RouterLink>
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Component-specific styles */
</style>
