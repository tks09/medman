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
}
</script>

<template>
  <nav class="bg-primary-600 text-white shadow-lg">
    <div class="container mx-auto px-4 py-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <RouterLink to="/" class="text-xl font-bold">{{ t('navbar.medicineManager') }}</RouterLink>
          <div v-if="authStore.isAuthenticated" class="hidden md:flex space-x-6">
            <RouterLink to="/dashboard" class="hover:text-primary-200">{{ t('navbar.dashboard') }}</RouterLink>
            <RouterLink to="/generate-plan" class="hover:text-primary-200">{{ t('navbar.generatePlan') }}</RouterLink>
          </div>
        </div>
        <div class="flex items-center space-x-4">
          <!-- Language Selector -->
          <div class="flex space-x-1">
            <button
              @click="changeLanguage('en')"
              class="px-2 py-1 text-xs rounded hover:bg-white hover:bg-opacity-20"
              :class="{ 'bg-white bg-opacity-30': locale === 'en' }"
            >
              EN
            </button>
            <button
              @click="changeLanguage('de')"
              class="px-2 py-1 text-xs rounded hover:bg-white hover:bg-opacity-20"
              :class="{ 'bg-white bg-opacity-30': locale === 'de' }"
            >
              DE
            </button>
            <button
              @click="changeLanguage('fr')"
              class="px-2 py-1 text-xs rounded hover:bg-white hover:bg-opacity-20"
              :class="{ 'bg-white bg-opacity-30': locale === 'fr' }"
            >
              FR
            </button>
          </div>
          <template v-if="authStore.isAuthenticated">
            <span class="text-sm">{{ t('navbar.welcome') }}, {{ authStore.user?.username }}</span>
            <button @click="logout" class="bg-white text-primary-600 px-3 py-1 rounded hover:bg-gray-100">
              {{ t('navbar.logout') }}
            </button>
          </template>
          <template v-else>
            <RouterLink to="/login" class="hover:text-primary-200">{{ t('navbar.login') }}</RouterLink>
            <RouterLink to="/register" class="bg-white text-primary-600 px-3 py-1 rounded hover:bg-gray-100">
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
  font-weight: 600;
  border-bottom: 2px solid white;
  padding-bottom: 2px;
}
</style>
