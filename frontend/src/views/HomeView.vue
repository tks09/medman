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
    <h1 class="text-4xl font-bold text-primary-600 mb-6">{{ t('home.title') }}</h1>
    <p class="text-xl text-gray-600 mb-8">
      {{ t('home.subtitle') }}
    </p>

    <div class="flex justify-center space-x-4">
      <template v-if="authStore.isAuthenticated">
        <RouterLink to="/dashboard" class="bg-primary-600 text-white px-6 py-3 rounded-lg hover:bg-primary-700 transition">
          {{ t('home.goToDashboard') }}
        </RouterLink>
        <RouterLink to="/generate-plan" class="bg-green-600 text-white px-6 py-3 rounded-lg hover:bg-green-700 transition">
          {{ t('home.generateNewPlan') }}
        </RouterLink>
      </template>
      <template v-else>
        <RouterLink to="/login" class="bg-primary-600 text-white px-6 py-3 rounded-lg hover:bg-primary-700 transition">
          {{ t('home.login') }}
        </RouterLink>
        <RouterLink to="/register" class="bg-green-600 text-white px-6 py-3 rounded-lg hover:bg-green-700 transition">
          {{ t('home.register') }}
        </RouterLink>
      </template>
    </div>

    <div class="mt-16 grid md:grid-cols-3 gap-8">
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h3 class="text-xl font-semibold text-primary-600 mb-3">{{ t('home.aiPoweredPlans') }}</h3>
        <p class="text-gray-600">{{ t('home.aiPoweredPlansDescription') }}</p>
      </div>
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h3 class="text-xl font-semibold text-primary-600 mb-3">{{ t('home.dailyTracking') }}</h3>
        <p class="text-gray-600">{{ t('home.dailyTrackingDescription') }}</p>
      </div>
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h3 class="text-xl font-semibold text-primary-600 mb-3">{{ t('home.personalizedInsights') }}</h3>
        <p class="text-gray-600">{{ t('home.personalizedInsightsDescription') }}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Add any component-specific styles here */
</style>
