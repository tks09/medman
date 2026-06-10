import { defineStore } from 'pinia'
import * as api from './services/api'

/* =========================================================
   UI  — theme, glass intensity, accent, language
   ========================================================= */
const ACCENTS = ['#12b5a6', '#2f7ef5', '#6c5ce7', '#f25f9c', '#1fae6a']

function loadUi() {
  try { return JSON.parse(localStorage.getItem('medman.ui')) || {} } catch { return {} }
}

export const useUi = defineStore('ui', {
  state: () => ({
    dark: false,
    glass: 62,
    accent: ACCENTS[0],
    lang: 'en',
    accents: ACCENTS,
    ...loadUi(),
  }),
  actions: {
    persist() {
      localStorage.setItem('medman.ui', JSON.stringify({ dark: this.dark, glass: this.glass, accent: this.accent, lang: this.lang }))
    },
    toggleTheme() { this.dark = !this.dark; this.apply() },
    setGlass(v) { this.glass = v; this.apply() },
    setAccent(v) { this.accent = v; this.apply() },
    setLang(v) { this.lang = v; this.persist() },
    apply() {
      const r = document.documentElement
      r.setAttribute('data-theme', this.dark ? 'dark' : 'light')
      r.style.setProperty('--accent', this.accent)
      const v = Math.max(0, Math.min(100, this.glass))
      r.style.setProperty('--glass-blur', (4 + v * 0.34).toFixed(1) + 'px')
      r.style.setProperty('--glass-sat', (110 + v * 0.9).toFixed(0) + '%')
      const lo = this.dark ? { a: 0.05, s: 0.11 } : { a: 0.40, s: 0.64 }
      const hi = this.dark ? { a: 0.17, s: 0.26 } : { a: 0.72, s: 0.90 }
      const f = v / 100
      r.style.setProperty('--surf-a', (lo.a + (hi.a - lo.a) * f).toFixed(3))
      r.style.setProperty('--surf-strong-a', (lo.s + (hi.s - lo.s) * f).toFixed(3))
      this.persist()
    },
  },
})

/* =========================================================
   AUTH
   ========================================================= */
export const useAuth = defineStore('auth', {
  state: () => {
    const saved = localStorage.getItem('medman.user')
    return { user: saved ? JSON.parse(saved) : null, loading: false }
  },
  getters: { isAuthed: (s) => !!s.user },
  actions: {
    async login(creds) {
      this.loading = true
      try {
        this.user = await api.login(creds)
        localStorage.setItem('medman.user', JSON.stringify(this.user))
      } finally { this.loading = false }
    },
    async signup(info) {
      this.loading = true
      try {
        this.user = await api.signup(info)
        localStorage.setItem('medman.user', JSON.stringify(this.user))
      } finally { this.loading = false }
    },
    async oauth(provider) {
      this.loading = true
      try {
        this.user = await api.oauth(provider)
        localStorage.setItem('medman.user', JSON.stringify(this.user))
      } finally { this.loading = false }
    },
    logout() {
      this.user = null
      localStorage.removeItem('medman.user')
    },
  },
})

/* =========================================================
   PLANS / CHECK-INS / INSIGHTS
   ========================================================= */
export const usePlans = defineStore('plans', {
  state: () => ({ plans: [], insights: [], mood: 'good', loaded: false }),
  getters: { active: (s) => s.plans[0] || null },
  actions: {
    async load() {
      const auth = useAuth()
      const token = auth.user?.token
      const [plans, insights] = await Promise.all([api.getPlans(token), api.getInsights(token)])
      this.plans = plans
      this.insights = insights
      this.loaded = true
    },
    async create(plan) {
      const auth = useAuth()
      const created = await api.createPlan(plan, auth.user?.token)
      this.plans = [created, ...this.plans]
      return created
    },
    setMood(m) { this.mood = m },
    async logCheckin(checkin) {
      const auth = useAuth()
      return api.logCheckin({ mood: this.mood, ...checkin }, auth.user?.token)
    },
  },
})
