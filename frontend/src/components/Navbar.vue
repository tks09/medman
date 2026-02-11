<script setup>
import { useAuthStore } from '../stores/auth'
import { useRouter } from 'vue-router'

const authStore = useAuthStore()
const router = useRouter()

const logout = () => {
  authStore.logout()
  router.push('/login')
}
</script>

<template>
  <nav class="bg-primary-600 text-white shadow-lg">
    <div class="container mx-auto px-4 py-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <RouterLink to="/" class="text-xl font-bold">Medicine Manager</RouterLink>
          <div v-if="authStore.isAuthenticated" class="hidden md:flex space-x-6">
            <RouterLink to="/dashboard" class="hover:text-primary-200">Dashboard</RouterLink>
            <RouterLink to="/generate-plan" class="hover:text-primary-200">Generate Plan</RouterLink>
          </div>
        </div>
        <div class="flex items-center space-x-4">
          <template v-if="authStore.isAuthenticated">
            <span class="text-sm">Welcome, {{ authStore.user?.username }}</span>
            <button @click="logout" class="bg-white text-primary-600 px-3 py-1 rounded hover:bg-gray-100">
              Logout
            </button>
          </template>
          <template v-else>
            <RouterLink to="/login" class="hover:text-primary-200">Login</RouterLink>
            <RouterLink to="/register" class="bg-white text-primary-600 px-3 py-1 rounded hover:bg-gray-100">
              Register
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
