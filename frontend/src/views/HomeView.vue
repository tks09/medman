<script setup>
import { useAuthStore } from '../stores/auth'
import { useRouter } from 'vue-router'

const authStore = useAuthStore()
const router = useRouter()
</script>

<template>
  <div class="max-w-4xl mx-auto text-center py-12">
    <h1 class="text-4xl font-bold text-primary-600 mb-6">Medicine Manager</h1>
    <p class="text-xl text-gray-600 mb-8">
      Track your medication effects and side effects with AI-powered insights
    </p>

    <div class="flex justify-center space-x-4">
      <template v-if="authStore.isAuthenticated">
        <RouterLink to="/dashboard" class="bg-primary-600 text-white px-6 py-3 rounded-lg hover:bg-primary-700 transition">
          Go to Dashboard
        </RouterLink>
        <RouterLink to="/generate-plan" class="bg-green-600 text-white px-6 py-3 rounded-lg hover:bg-green-700 transition">
          Generate New Plan
        </RouterLink>
      </template>
      <template v-else>
        <RouterLink to="/login" class="bg-primary-600 text-white px-6 py-3 rounded-lg hover:bg-primary-700 transition">
          Login
        </RouterLink>
        <RouterLink to="/register" class="bg-green-600 text-white px-6 py-3 rounded-lg hover:bg-green-700 transition">
          Register
        </RouterLink>
      </template>
    </div>

    <div class="mt-16 grid md:grid-cols-3 gap-8">
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h3 class="text-xl font-semibold text-primary-600 mb-3">AI-Powered Plans</h3>
        <p class="text-gray-600">Generate comprehensive medication review plans using Mistral AI</p>
      </div>
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h3 class="text-xl font-semibold text-primary-600 mb-3">Daily Tracking</h3>
        <p class="text-gray-600">Track symptoms, side effects, and progress daily</p>
      </div>
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h3 class="text-xl font-semibold text-primary-600 mb-3">Personalized Insights</h3>
        <p class="text-gray-600">Get insights tailored to your specific medication and concerns</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Add any component-specific styles here */
</style>
