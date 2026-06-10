// ============================================================
//  services/api.js  —  THE BACKEND SEAM
// ============================================================

const USE_MOCK = false
const API_BASE = import.meta.env.VITE_API_BASE || '/api'

async function http(path, { method = 'GET', body, token } = {}) {
  const res = await fetch(API_BASE + path, {
    method,
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    body: body ? JSON.stringify(body) : undefined,
  })
  if (!res.ok) throw new Error(`${method} ${path} → ${res.status}`)
  return res.status === 204 ? null : res.json()
}

/* ---- AUTH ---- */

// POST /auth/login   { email, password } -> { id, name, email, token }
export async function login({ email, password }) {
  if (USE_MOCK) return { id: 'u_1', name: 'Alex', email, token: 'mock-token' }
  return http('/auth/login', { method: 'POST', body: { email, password } })
}

// POST /auth/signup  { name, email, password } -> { id, name, email, token }
export async function signup({ name, email, password }) {
  if (USE_MOCK) return { id: 'u_1', name, email, token: 'mock-token' }
  return http('/auth/signup', { method: 'POST', body: { name, email, password } })
}

// POST /auth/oauth   { provider } -> { id, name, email, token }
export async function oauth(provider) {
  if (USE_MOCK) return { id: 'u_1', name: 'Alex', email: 'alex@medman.app', token: 'mock-token', provider }
  return http('/auth/oauth', { method: 'POST', body: { provider } })
}

/* ---- PLANS ---- */

// GET /plans -> Plan[]
export async function getPlans(token) {
  if (USE_MOCK) return [{ id: 'p_1', name: 'Sertraline', dose: '50 mg · mornings', focus: ['Mood', 'Sleep'], adherence: 86 }]
  return http('/plans', { token })
}

// POST /plans  { name, dose, focus[], adherence } -> Plan
export async function createPlan(plan, token) {
  if (USE_MOCK) return { id: 'p_' + Date.now(), adherence: 0, ...plan }
  return http('/plans', { method: 'POST', body: plan, token })
}

// GET /plans/:id/series -> number[]   (wellbeing scores over time)
export async function getPlanSeries(id, token) {
  if (USE_MOCK) return [3, 2, 3, 4, 3, 4, 5, 4, 4, 5, 4, 5, 5]
  return http(`/plans/${id}/series`, { token })
}

/* ---- CHECK-INS ---- */

// POST /checkins  { mood, sideEffects[], note, planId } -> Checkin
export async function logCheckin(checkin, token) {
  if (USE_MOCK) return { id: 'c_' + Date.now(), date: new Date().toISOString(), ...checkin }
  return http('/checkins', { method: 'POST', body: checkin, token })
}

/* ---- INSIGHTS ---- */

// GET /insights -> Insight[]
export async function getInsights(token) {
  if (USE_MOCK) return [
    { id: 'i_1', icon: 'spark', tone: 'ai', title: 'Mood is trending up', body: 'Your "Good" days rose 24% this week. Mornings after 7h+ sleep score highest.' },
    { id: 'i_2', icon: 'activity', tone: 'accent', title: 'Fewer stomach flags', body: 'Stomach side-effects dropped 60% on days you logged a meal before your dose.' },
  ]
  return http('/insights', { token })
}

/* ---- ASSISTANT ---- */

// POST /assistant/plan-chat
//   { messages:[{role,content}], draft:{...} }
//   -> { reply, draft, complete }
export async function planChat(payload, token) {
  if (USE_MOCK) return { reply: '…', draft: payload.draft, complete: false }
  return http('/assistant/plan-chat', { method: 'POST', body: payload, token })
}
