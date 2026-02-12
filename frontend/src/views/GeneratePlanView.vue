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
    <h1 class="text-3xl font-bold text-primary-600 mb-6">{{ t('generatePlan.title') }}</h1>

    <div v-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6">
      {{ error }}
    </div>

    <div v-if="!generatedPlan" class="bg-white p-6 rounded-lg shadow-md">
      <form @submit.prevent="generatePlan" class="space-y-6">
        <div>
          <label for="medicationName" class="block text-sm font-medium text-gray-700 mb-1">
            {{ t('generatePlan.medicationName') }}
          </label>
          <input
            id="medicationName"
            v-model="medicationName"
            type="text"
            :placeholder="t('generatePlan.medicationNamePlaceholder')"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
          />
          <p class="text-sm text-gray-500 mt-1">
            {{ t('generatePlan.medicationNameHelper') }}
          </p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
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
              class="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
            <button
              type="button"
              @click="removeFocusArea(index)"
              class="text-red-500 hover:text-red-700 text-xl font-bold"
              :disabled="focusAreas.length === 1"
            >
              ×
            </button>
          </div>

          <button
            type="button"
            @click="addFocusArea"
            class="text-primary-600 hover:text-primary-700 text-sm flex items-center"
          >
            <span class="mr-1">+</span> {{ t('generatePlan.addFocusArea') }}
          </button>
        </div>

        <div class="flex justify-end">
          <button
            type="submit"
            :disabled="isLoading"
            class="bg-primary-600 text-white px-6 py-2 rounded-md hover:bg-primary-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="isLoading">{{ t('generatePlan.generatingPlan') }}</span>
            <span v-else>{{ t('generatePlan.generatePlan') }}</span>
          </button>
        </div>
      </form>
    </div>

    <div v-else class="bg-white p-6 rounded-lg shadow-md mt-6">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-xl font-semibold text-primary-600">{{ t('generatePlan.generatedPlan') }}</h2>
        <button
          @click="goToReview"
          class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 transition"
        >
          {{ t('generatePlan.startDailyReview') }}
        </button>
      </div>

      <div class="prose max-w-none">
        <h3 class="font-semibold mt-4 mb-2">{{ t('generatePlan.medication') }} {{ generatedPlan.medication_name }}</h3>
        <h4 class="font-medium mb-2">{{ t('generatePlan.focusAreasLabel') }}</h4>
        <ul class="list-disc list-inside mb-4">
          <li v-for="area in generatedPlan.focus_areas" :key="area">{{ area }}</li>
        </ul>

        <div class="bg-gray-50 p-4 rounded-lg">
          <pre class="whitespace-pre-wrap text-sm">{{ generatedPlan.plan_content }}</pre>
        </div>
      </div>

      <div class="mt-6 flex justify-end">
        <button
          @click="generatedPlan = null"
          class="text-gray-600 hover:text-gray-800 mr-4"
        >
          {{ t('generatePlan.generateAnotherPlan') }}
        </button>
        <button
          @click="goToReview"
          class="bg-primary-600 text-white px-4 py-2 rounded hover:bg-primary-700 transition"
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
