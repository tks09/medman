import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
})

// Add request interceptor for JWT token
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
}, (error) => {
  return Promise.reject(error)
})

export default {
  // Auth endpoints
  async register(username, password) {
    try {
      const response = await api.post('/auth/register', { username, password })
      return response.data
    } catch (error) {
      throw error.response?.data || error.message
    }
  },

  async login(username, password) {
    try {
      const response = await api.post('/auth/login', { username, password })
      return response.data
    } catch (error) {
      throw error.response?.data || error.message
    }
  },

  // Medication endpoints
  async generatePlan(userId, medicationName, focusAreas) {
    try {
      const response = await api.post('/medication/plans', {
        user_id: userId,
        medication_name: medicationName,
        focus_areas: focusAreas,
      })
      return response.data
    } catch (error) {
      throw error.response?.data || error.message
    }
  },

  async getReviews(userId) {
    try {
      const response = await api.get('/medication/reviews', {
        params: { user_id: userId }
      })
      return response.data
    } catch (error) {
      throw error.response?.data || error.message
    }
  },

  async createReview(reviewData) {
    try {
      const response = await api.post('/medication/reviews', reviewData)
      return response.data
    } catch (error) {
      throw error.response?.data || error.message
    }
  },

  // Health check
  async healthCheck() {
    try {
      const response = await api.get('/health')
      return response.data
    } catch (error) {
      throw error.response?.data || error.message
    }
  },
}
