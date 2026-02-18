<script setup>
import { useAuthStore } from '../stores/auth'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'

const authStore = useAuthStore()
const router = useRouter()
const { t, locale } = useI18n()

const logout = () => {
  authStore.logout()
  router.push('/login')
}

const changeLanguage = (lang) => {
  locale.value = lang
  localStorage.setItem('locale', lang)
}
</script>

<template>
  <nav class="bg-navy-950 text-white shadow-lg border-b border-navy-700">
    <div class="container mx-auto px-4 py-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <RouterLink to="/" class="text-xl font-bold text-primary-400 hover:text-primary-300 transition-colors">
            {{ t('navbar.medicineManager') }}
          </RouterLink>
          <div v-if="authStore.isAuthenticated" class="hidden md:flex space-x-6">
            <RouterLink to="/dashboard" class="text-gray-300 hover:text-primary-400 transition-colors">{{ t('navbar.dashboard') }}</RouterLink>
            <RouterLink to="/generate-plan" class="text-gray-300 hover:text-primary-400 transition-colors">{{ t('navbar.generatePlan') }}</RouterLink>
          </div>
        </div>
        <div class="flex items-center space-x-4">
          <!-- Language Selector -->
          <div class="flex space-x-1 bg-navy-800 rounded p-1 border border-navy-600">
            <button
              @click="changeLanguage('en')"
              class="px-3 py-1 text-xs rounded transition-all duration-200"
              :class="{ 'bg-primary-600 text-white': locale === 'en', 'text-gray-400 hover:text-gray-200': locale !== 'en' }"
            >
              EN
            </button>
            <button
              @click="changeLanguage('de')"
              class="px-3 py-1 text-xs rounded transition-all duration-200"
              :class="{ 'bg-primary-600 text-white': locale === 'de', 'text-gray-400 hover:text-gray-200': locale !== 'de' }"
            >
              DE
            </button>
            <button
              @click="changeLanguage('fr')"
              class="px-3 py-1 text-xs rounded transition-all duration-200"
              :class="{ 'bg-primary-600 text-white': locale === 'fr', 'text-gray-400 hover:text-gray-200': locale !== 'fr' }"
            >
              FR
            </button>
          </div>
          <template v-if="authStore.isAuthenticated">
            <span class="text-sm text-gray-300">{{ t('navbar.welcome') }}, {{ authStore.user?.username }}</span>
            <button @click="logout" class="bg-navy-700 text-gray-200 px-4 py-2 rounded-lg hover:bg-navy-600 hover:text-white transition-all border border-navy-500">
              {{ t('navbar.logout') }}
            </button>
          </template>
          <template v-else>
            <RouterLink to="/login" class="text-gray-300 hover:text-primary-400 transition-colors">{{ t('navbar.login') }}</RouterLink>
            <RouterLink to="/register" class="bg-primary-600 text-white px-4 py-2 rounded-lg hover:bg-primary-500 transition-all shadow-sm">
              {{ t('navbar.register') }}
            </RouterLink>
          </template>
        </div>
      </div>
    </div>
  </nav>
</template>

<style scoped>
.router-link-active {
  color: var(--color-primary-400);
  font-weight: 600;
  border-bottom: 2px solid var(--color-primary-500);
  padding-bottom: 2px;
}
</style>
