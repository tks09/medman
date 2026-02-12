<script setup>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import api from '../services/api'
import { useI18n } from 'vue-i18n'

const authStore = useAuthStore()
const { t } = useI18n()
const plans = ref([])
const reviews = ref([])
const isLoading = ref(false)
const error = ref(null)

onMounted(async () => {
  await fetchData()
})

const fetchData = async () => {
  try {
    isLoading.value = true
    error.value = null

    // Fetch medication plans
    // TODO: Implement proper API endpoint for getting user plans

    // Fetch medication reviews
    const reviewsData = await api.getReviews(authStore.userId)
    reviews.value = reviewsData

  } catch (err) {
    error.value = err.error || err.message || t('dashboard.error')
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="max-w-6xl mx-auto">
    <h1 class="text-3xl font-bold text-primary-600 mb-6">{{ t('dashboard.title') }}</h1>

    <div v-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6">
      {{ error }}
    </div>

    <div class="grid md:grid-cols-2 gap-8">
      <!-- Medication Plans Section -->
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h2 class="text-xl font-semibold text-primary-600 mb-4">{{ t('dashboard.yourMedicationPlans') }}</h2>

        <div v-if="isLoading" class="text-center py-4">
          <p class="text-gray-600">{{ t('dashboard.loadingPlans') }}</p>
        </div>

        <div v-else>
          <div v-if="plans.length === 0" class="text-center py-4">
            <p class="text-gray-600">{{ t('dashboard.noPlansYet') }}</p>
            <RouterLink to="/generate-plan" class="text-primary-600 hover:underline">
              {{ t('dashboard.generateFirstPlan') }}
            </RouterLink>
          </div>

          <div v-else class="space-y-4">
            <div v-for="plan in plans" :key="plan.id" class="border border-gray-200 rounded-lg p-4">
              <h3 class="font-medium text-primary-600">{{ plan.medication_name }}</h3>
              <p class="text-sm text-gray-600 mt-1">{{ t('dashboard.created') }} {{ new Date(plan.created_at).toLocaleDateString() }}</p>
              <div class="mt-3 flex space-x-2">
                <RouterLink
                  :to="`/review/${plan.id}`"
                  class="text-sm bg-primary-600 text-white px-3 py-1 rounded hover:bg-primary-700"
                >
                  {{ t('dashboard.addReview') }}
                </RouterLink>
                <button class="text-sm bg-gray-200 text-gray-700 px-3 py-1 rounded hover:bg-gray-300">
                  {{ t('dashboard.viewPlan') }}
                </button>
              </div>
            </div>
          </div>
        </div>

        <div class="mt-6">
          <RouterLink to="/generate-plan" class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 transition">
            {{ t('dashboard.generateNewPlan') }}
          </RouterLink>
        </div>
      </div>

      <!-- Recent Reviews Section -->
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h2 class="text-xl font-semibold text-primary-600 mb-4">{{ t('dashboard.recentReviews') }}</h2>

        <div v-if="isLoading" class="text-center py-4">
          <p class="text-gray-600">{{ t('dashboard.loadingReviews') }}</p>
        </div>

        <div v-else>
          <div v-if="reviews.length === 0" class="text-center py-4">
            <p class="text-gray-600">{{ t('dashboard.noReviewsYet') }}</p>
          </div>

          <div v-else class="space-y-4 max-h-96 overflow-y-auto">
            <div v-for="review in reviews.slice(0, 5)" :key="review.id" class="border border-gray-200 rounded-lg p-4">
              <div class="flex justify-between items-start">
                <div>
                  <p class="text-sm text-gray-500">
                    {{ new Date(review.date).toLocaleDateString() }}
                  </p>
                  <p class="font-medium mt-1">{{ t('dashboard.rating') }} {{ review.rating }}/10</p>
                </div>
                <span class="text-xs bg-primary-100 text-primary-600 px-2 py-1 rounded-full">
                  {{ review.side_effects ? t('dashboard.sideEffectsReported') : t('dashboard.noSideEffects') }}
                </span>
              </div>
              <p class="text-sm text-gray-600 mt-2">{{ review.notes }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Component-specific styles */
</style>
