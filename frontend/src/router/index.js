import { createRouter, createWebHashHistory } from 'vue-router'
import { useAuth } from '../stores'

const routes = [
  { path: '/login', name: 'login', component: () => import('../views/LoginView.vue'), meta: { public: true } },
  { path: '/', name: 'home', component: () => import('../views/HomeView.vue') },
  { path: '/chat', name: 'chat', component: () => import('../views/ChatView.vue') },
  { path: '/plan/:id?', name: 'plan', component: () => import('../views/PlanView.vue') },
  { path: '/track', name: 'track', component: () => import('../views/TrackView.vue') },
  { path: '/insights', name: 'insights', component: () => import('../views/InsightsView.vue') },
  { path: '/:pathMatch(.*)*', redirect: '/' },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

router.beforeEach((to) => {
  const auth = useAuth()
  if (!to.meta.public && !auth.isAuthed) return { name: 'login' }
  if (to.name === 'login' && auth.isAuthed) return { name: 'home' }
})

export default router
