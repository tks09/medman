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
    <h1 class="text-3xl font-bold text-primary-400 mb-6">{{ t('dashboard.title') }}</h1>

    <div v-if="error" class="bg-red-950 border border-red-700 text-red-300 px-4 py-3 rounded mb-6">
      {{ error }}
    </div>

    <div class="grid md:grid-cols-2 gap-8">
      <!-- Medication Plans Section -->
      <div class="bg-navy-800 p-6 rounded-lg shadow-lg border border-navy-700">
        <h2 class="text-xl font-semibold text-primary-400 mb-4">{{ t('dashboard.yourMedicationPlans') }}</h2>

        <div v-if="isLoading" class="text-center py-4">
          <p class="text-gray-400">{{ t('dashboard.loadingPlans') }}</p>
        </div>

        <div v-else>
          <div v-if="plans.length === 0" class="text-center py-4">
            <p class="text-gray-400">{{ t('dashboard.noPlansYet') }}</p>
            <RouterLink to="/generate-plan" class="text-primary-400 hover:text-primary-300 hover:underline transition-colors">
              {{ t('dashboard.generateFirstPlan') }}
            </RouterLink>
          </div>

          <div v-else class="space-y-4">
            <div v-for="plan in plans" :key="plan.id" class="border border-navy-600 rounded-lg p-4 hover:border-primary-600 hover:shadow-md transition-all">
              <h3 class="font-medium text-primary-400">{{ plan.medication_name }}</h3>
              <p class="text-sm text-gray-400 mt-1">{{ t('dashboard.created') }} {{ new Date(plan.created_at).toLocaleDateString() }}</p>
              <div class="mt-3 flex space-x-2">
                <RouterLink
                  :to="`/review/${plan.id}`"
                  class="text-sm bg-primary-600 text-white px-3 py-1 rounded hover:bg-primary-500 transition-all"
                >
                  {{ t('dashboard.addReview') }}
                </RouterLink>
                <button class="text-sm bg-navy-700 text-gray-300 px-3 py-1 rounded hover:bg-navy-600 transition-all">
                  {{ t('dashboard.viewPlan') }}
                </button>
              </div>
            </div>
          </div>
        </div>

        <div class="mt-6">
          <RouterLink to="/generate-plan" class="bg-warm-600 text-white px-4 py-2 rounded-lg hover:bg-warm-500 transition-all shadow-lg">
            {{ t('dashboard.generateNewPlan') }}
          </RouterLink>
        </div>
      </div>

      <!-- Recent Reviews Section -->
      <div class="bg-navy-800 p-6 rounded-lg shadow-lg border border-navy-700">
        <h2 class="text-xl font-semibold text-primary-400 mb-4">{{ t('dashboard.recentReviews') }}</h2>

        <div v-if="isLoading" class="text-center py-4">
          <p class="text-gray-400">{{ t('dashboard.loadingReviews') }}</p>
        </div>

        <div v-else>
          <div v-if="reviews.length === 0" class="text-center py-4">
            <p class="text-gray-400">{{ t('dashboard.noReviewsYet') }}</p>
          </div>

          <div v-else class="space-y-4 max-h-96 overflow-y-auto">
            <div v-for="review in reviews.slice(0, 5)" :key="review.id" class="border border-navy-600 rounded-lg p-4 hover:border-primary-600 hover:shadow-md transition-all">
              <div class="flex justify-between items-start">
                <div>
                  <p class="text-sm text-gray-500">
                    {{ new Date(review.date).toLocaleDateString() }}
                  </p>
                  <p class="font-medium text-gray-200 mt-1">{{ t('dashboard.rating') }} {{ review.rating }}/10</p>
                </div>
                <span class="text-xs bg-primary-600/20 text-primary-400 px-2 py-1 rounded-full border border-primary-700">
                  {{ review.side_effects ? t('dashboard.sideEffectsReported') : t('dashboard.noSideEffects') }}
                </span>
              </div>
              <p class="text-sm text-gray-400 mt-2">{{ review.notes }}</p>
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
