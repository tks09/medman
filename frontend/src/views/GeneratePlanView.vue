<script setup>
import { ref } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useRouter } from 'vue-router'
import api from '../services/api'
import { useI18n } from 'vue-i18n'

const authStore = useAuthStore()
const router = useRouter()
const { t } = useI18n()

const medicationName = ref('')
const focusAreas = ref([''])
const isLoading = ref(false)
const error = ref(null)
const generatedPlan = ref(null)

const addFocusArea = () => {
  focusAreas.value.push('')
}

const removeFocusArea = (index) => {
  if (focusAreas.value.length > 1) {
    focusAreas.value.splice(index, 1)
  }
}

const generatePlan = async () => {
  try {
    error.value = null
    isLoading.value = true

    // Filter out empty focus areas
    const validFocusAreas = focusAreas.value.filter(area => area.trim() !== '')

    if (!medicationName.value.trim()) {
      error.value = t('generatePlan.pleaseEnterMedicationName')
      return
    }

    if (validFocusAreas.length === 0) {
      error.value = t('generatePlan.pleaseAddFocusArea')
      return
    }

    const response = await api.generatePlan(
      authStore.userId,
      medicationName.value.trim(),
      validFocusAreas
    )

    generatedPlan.value = response

  } catch (err) {
    error.value = err.error || err.message || t('generatePlan.error')
  } finally {
    isLoading.value = false
  }
}

const goToReview = () => {
  if (generatedPlan.value && generatedPlan.value.id) {
    router.push(`/review/${generatedPlan.value.id}`)
  }
}
</script>

<template>
  <div class="max-w-4xl mx-auto">
    <h1 class="text-3xl font-bold text-primary-400 mb-6">{{ t('generatePlan.title') }}</h1>

    <div v-if="error" class="bg-red-950 border border-red-700 text-red-300 px-4 py-3 rounded mb-6">
      {{ error }}
    </div>

    <div v-if="!generatedPlan" class="bg-navy-800 p-6 rounded-lg shadow-lg border border-navy-700">
      <form @submit.prevent="generatePlan" class="space-y-6">
        <div>
          <label for="medicationName" class="block text-sm font-medium text-gray-300 mb-1">
            {{ t('generatePlan.medicationName') }}
          </label>
          <input
            id="medicationName"
            v-model="medicationName"
            type="text"
            :placeholder="t('generatePlan.medicationNamePlaceholder')"
            required
            class="w-full px-3 py-2 bg-navy-900 border border-navy-600 text-gray-100 placeholder-gray-500 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-all"
          />
          <p class="text-sm text-gray-500 mt-1">
            {{ t('generatePlan.medicationNameHelper') }}
          </p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-300 mb-1">
            {{ t('generatePlan.focusAreas') }}
          </label>
          <p class="text-sm text-gray-500 mb-2">
            {{ t('generatePlan.focusAreasHelper') }}
          </p>

          <div v-for="(area, index) in focusAreas" :key="index" class="flex items-center space-x-2 mb-2">
            <input
              v-model="focusAreas[index]"
              type="text"
              :placeholder="t('generatePlan.focusAreaPlaceholder')"
              class="flex-1 px-3 py-2 bg-navy-900 border border-navy-600 text-gray-100 placeholder-gray-500 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-all"
            />
            <button
              type="button"
              @click="removeFocusArea(index)"
              class="text-red-400 hover:text-red-300 text-xl font-bold transition-colors"
              :disabled="focusAreas.length === 1"
            >
              ×
            </button>
          </div>

          <button
            type="button"
            @click="addFocusArea"
            class="text-primary-400 hover:text-primary-300 text-sm flex items-center transition-colors"
          >
            <span class="mr-1">+</span> {{ t('generatePlan.addFocusArea') }}
          </button>
        </div>

        <div class="flex justify-end">
          <button
            type="submit"
            :disabled="isLoading"
            class="bg-primary-600 text-white px-6 py-3 rounded-lg hover:bg-primary-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-lg hover:shadow-xl"
          >
            <span v-if="isLoading">{{ t('generatePlan.generatingPlan') }}</span>
            <span v-else>{{ t('generatePlan.generatePlan') }}</span>
          </button>
        </div>
      </form>
    </div>

    <div v-else class="bg-navy-800 p-6 rounded-lg shadow-lg border border-navy-700 mt-6">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-xl font-semibold text-primary-400">{{ t('generatePlan.generatedPlan') }}</h2>
        <button
          @click="goToReview"
          class="bg-warm-600 text-white px-4 py-2 rounded-lg hover:bg-warm-500 transition-all shadow-lg"
        >
          {{ t('generatePlan.startDailyReview') }}
        </button>
      </div>

      <div class="prose max-w-none">
        <h3 class="font-semibold text-gray-200 mt-4 mb-2">{{ t('generatePlan.medication') }} {{ generatedPlan.medication_name }}</h3>
        <h4 class="font-medium text-gray-300 mb-2">{{ t('generatePlan.focusAreasLabel') }}</h4>
        <ul class="list-disc list-inside mb-4 text-gray-400">
          <li v-for="area in generatedPlan.focus_areas" :key="area">{{ area }}</li>
        </ul>

        <div class="bg-navy-900 border border-navy-600 p-4 rounded-lg">
          <pre class="whitespace-pre-wrap text-sm text-gray-300">{{ generatedPlan.plan_content }}</pre>
        </div>
      </div>

      <div class="mt-6 flex justify-end">
        <button
          @click="generatedPlan = null"
          class="text-gray-400 hover:text-gray-200 mr-4 transition-colors"
        >
          {{ t('generatePlan.generateAnotherPlan') }}
        </button>
        <button
          @click="goToReview"
          class="bg-primary-600 text-white px-4 py-2 rounded-lg hover:bg-primary-500 transition-all shadow-lg"
        >
          {{ t('generatePlan.startDailyReview') }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Component-specific styles */
</style>
