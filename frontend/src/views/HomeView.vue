<script setup>
import { useAuthStore } from '../stores/auth'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'

const authStore = useAuthStore()
const router = useRouter()
const { t } = useI18n()
</script>

<template>
  <div class="max-w-4xl mx-auto text-center py-12">
    <h1 class="text-4xl font-bold text-primary-400 mb-6">{{ t('home.title') }}</h1>
    <p class="text-xl text-gray-400 mb-8">
      {{ t('home.subtitle') }}
    </p>

    <div class="flex justify-center space-x-4">
      <template v-if="authStore.isAuthenticated">
        <RouterLink to="/dashboard" class="bg-primary-600 text-white px-6 py-3 rounded-lg hover:bg-primary-500 transition-all shadow-lg">
          {{ t('home.goToDashboard') }}
        </RouterLink>
        <RouterLink to="/generate-plan" class="bg-warm-600 text-white px-6 py-3 rounded-lg hover:bg-warm-500 transition-all shadow-lg">
          {{ t('home.generateNewPlan') }}
        </RouterLink>
      </template>
      <template v-else>
        <RouterLink to="/login" class="bg-primary-600 text-white px-6 py-3 rounded-lg hover:bg-primary-500 transition-all shadow-lg">
          {{ t('navbar.login') }}
        </RouterLink>
        <RouterLink to="/register" class="bg-warm-600 text-white px-6 py-3 rounded-lg hover:bg-warm-500 transition-all shadow-lg">
          {{ t('navbar.register') }}
        </RouterLink>
      </template>
    </div>

    <div class="mt-16 grid md:grid-cols-3 gap-8">
      <div class="bg-navy-800 p-6 rounded-lg shadow-lg hover:shadow-xl hover:border-primary-600 border border-navy-700 transition-all duration-300">
        <div class="w-12 h-12 bg-primary-600/20 text-primary-400 rounded-full flex items-center justify-center mx-auto mb-4">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"></path>
          </svg>
        </div>
        <h3 class="text-xl font-semibold text-primary-400 mb-3">{{ t('home.aiPoweredPlans') }}</h3>
        <p class="text-gray-400">{{ t('home.aiPoweredPlansDescription') }}</p>
      </div>
      <div class="bg-navy-800 p-6 rounded-lg shadow-lg hover:shadow-xl hover:border-primary-600 border border-navy-700 transition-all duration-300">
        <div class="w-12 h-12 bg-primary-600/20 text-primary-400 rounded-full flex items-center justify-center mx-auto mb-4">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
          </svg>
        </div>
        <h3 class="text-xl font-semibold text-primary-400 mb-3">{{ t('home.dailyTracking') }}</h3>
        <p class="text-gray-400">{{ t('home.dailyTrackingDescription') }}</p>
      </div>
      <div class="bg-navy-800 p-6 rounded-lg shadow-lg hover:shadow-xl hover:border-primary-600 border border-navy-700 transition-all duration-300">
        <div class="w-12 h-12 bg-primary-600/20 text-primary-400 rounded-full flex items-center justify-center mx-auto mb-4">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
          </svg>
        </div>
        <h3 class="text-xl font-semibold text-primary-400 mb-3">{{ t('home.personalizedInsights') }}</h3>
        <p class="text-gray-400">{{ t('home.personalizedInsightsDescription') }}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Add any component-specific styles here */
</style>
