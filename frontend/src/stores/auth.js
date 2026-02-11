import { defineStore } from 'pinia'
import api from '../services/api'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null,
    token: localStorage.getItem('token') || null,
    userId: localStorage.getItem('userId') || null,
  }),

  actions: {
    async register(username, password) {
      try {
        const response = await api.register(username, password)
        this.token = response.token
        this.userId = response.user_id
        this.user = { username, id: response.user_id }

        localStorage.setItem('token', this.token)
        localStorage.setItem('userId', this.userId)

        return response
      } catch (error) {
        throw error
      }
    },

    async login(username, password) {
      try {
        const response = await api.login(username, password)
        this.token = response.token
        this.userId = response.user_id
        this.user = { username, id: response.user_id }

        localStorage.setItem('token', this.token)
        localStorage.setItem('userId', this.userId)

        return response
      } catch (error) {
        throw error
      }
    },

    logout() {
      this.user = null
      this.token = null
      this.userId = null

      localStorage.removeItem('token')
      localStorage.removeItem('userId')
    },

    initialize() {
      if (this.token && this.userId) {
        // User is already authenticated
        return true
      }
      return false
    }
  },

  getters: {
    isAuthenticated: (state) => !!state.token,
  }
})
