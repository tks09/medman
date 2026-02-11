<script setup>
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import api from '../services/api'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

const planId = route.params.planId
const date = ref(new Date().toISOString().split('T')[0])
const symptoms = ref('')
const sideEffects = ref('')
const notes = ref('')
const rating = ref(5)
const isLoading = ref(false)
const error = ref(null)
const success = ref(false)

const submitReview = async () => {
  try {
    error.value = null
    isLoading.value = true

    const reviewData = {
      user_id: authStore.userId,
      plan_id: planId,
      date: new Date(date.value).toISOString(),
      symptoms: symptoms.value,
      side_effects: sideEffects.value,
      notes: notes.value,
      rating: rating.value,
    }

    await api.createReview(reviewData)

    success.value = true

    // Reset form after 2 seconds
    setTimeout(() => {
      success.value = false
      router.push('/dashboard')
    }, 2000)

  } catch (err) {
    error.value = err.error || err.message || 'Failed to submit review'
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="max-w-3xl mx-auto">
    <h1 class="text-3xl font-bold text-primary-600 mb-6">Daily Medication Review</h1>

    <div v-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6">
      {{ error }}
    </div>

    <div v-if="success" class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-6">
      Review submitted successfully! Redirecting to dashboard...
    </div>

    <div class="bg-white p-6 rounded-lg shadow-md">
      <form @submit.prevent="submitReview" class="space-y-6">
        <div>
          <label for="date" class="block text-sm font-medium text-gray-700 mb-1">Date</label>
          <input
            id="date"
            v-model="date"
            type="date"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
          />
        </div>

        <div>
          <label for="symptoms" class="block text-sm font-medium text-gray-700 mb-1">
            Symptoms Experienced
          </label>
          <textarea
            id="symptoms"
            v-model="symptoms"
            rows="3"
            placeholder="Describe any symptoms you experienced (e.g., headache, nausea, fatigue)"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
          ></textarea>
        </div>

        <div>
          <label for="sideEffects" class="block text-sm font-medium text-gray-700 mb-1">
            Side Effects
          </label>
          <textarea
            id="sideEffects"
            v-model="sideEffects"
            rows="3"
            placeholder="Describe any side effects, especially related to your focus areas"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
          ></textarea>
        </div>

        <div>
          <label for="notes" class="block text-sm font-medium text-gray-700 mb-1">
            Additional Notes
          </label>
          <textarea
            id="notes"
            v-model="notes"
            rows="3"
            placeholder="Any other observations or notes about your medication experience"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
          ></textarea>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Overall Rating (1-10)
          </label>
          <input
            v-model.number="rating"
            type="range"
            min="1"
            max="10"
            class="w-full"
          />
          <div class="text-center mt-1">
            <span class="font-medium">{{ rating }}</span>
          </div>
        </div>

        <div class="flex justify-end">
          <button
            type="submit"
            :disabled="isLoading"
            class="bg-primary-600 text-white px-6 py-2 rounded-md hover:bg-primary-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="isLoading">Submitting...</span>
            <span v-else>Submit Review</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<style scoped>
/* Component-specific styles */
</style>
