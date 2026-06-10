<script setup>
import { ref, computed } from 'vue'
import { useAuth, useUi } from '../stores'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'

const route = useRoute()
const auth = useAuth()
const ui = useUi()
const router = useRouter()
const { locale } = useI18n()

const showHeader = computed(() => !['login', 'register'].includes(route.name))
const showDropdown = ref(false)

const logout = () => {
  auth.logout()
  router.push('/login')
  showDropdown.value = false
}

const changeLanguage = (lang) => {
  locale.value = lang
  localStorage.setItem('locale', lang)
  showDropdown.value = false
}

const toggleTheme = () => {
  ui.toggleTheme()
  showDropdown.value = false
}
</script>

<template>
  <div v-if="showHeader" style="padding: 8px 18px; position: relative; z-index: 40; display: flex; justify-content: space-between; align-items: center;">
    <span class="h-md nowrap">MedMan</span>
    <div style="position: relative;">
      <div v-if="auth.isAuthed" class="avatar" @click="showDropdown = !showDropdown" style="cursor: pointer;">
        {{ auth.user?.name?.charAt(0).toUpperCase() || 'U' }}
      </div>
      <div v-else class="avatar" @click="router.push('/login')" style="cursor: pointer;">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
          <circle cx="9" cy="7" r="4"/>
          <path d="M22 21v-2a4 4 0 0 0-3-3.5"/>
          <path d="M15 10l2 2"/>
        </svg>
      </div>
      <div v-if="showDropdown && auth.isAuthed" style="position: absolute; top: 100%; right: 0; margin-top: 8px; min-width: 200px; background: rgba(var(--surf-rgb)/var(--surf-strong-a)); backdrop-filter: blur(var(--glass-blur)); border-radius: var(--radius-md); border: 1px solid var(--hair); box-shadow: var(--shadow); z-index: 50; padding: 8px;">
        <div style="padding: 6px 0; border-bottom: 1px solid var(--hair-edge); margin-bottom: 4px;">
          <span style="display: block; font-size: 12px; color: var(--muted); margin-bottom: 2px;" v-if="auth.user?.name">Signed in as</span>
          <span style="font-size: 14px; font-weight: 600; color: var(--ink);">{{ auth.user?.name }}</span>
        </div>
        <div style="display: flex; gap: 4px; margin-bottom: 8px;">
          <button @click="changeLanguage('en')" class="chip" :class="{ sel: locale === 'en' }" style="flex: 1; font-size: 12px; padding: 6px;">EN</button>
          <button @click="changeLanguage('de')" class="chip" :class="{ sel: locale === 'de' }" style="flex: 1; font-size: 12px; padding: 6px;">DE</button>
          <button @click="changeLanguage('fr')" class="chip" :class="{ sel: locale === 'fr' }" style="flex: 1; font-size: 12px; padding: 6px;">FR</button>
        </div>
        <button @click="toggleTheme" class="btn block" style="margin-bottom: 8px; font-size: 13px; padding: 10px;">
          {{ ui.dark ? 'Light Mode' : 'Dark Mode' }}
        </button>
        <button @click="logout" class="btn" style="font-size: 13px; padding: 10px; width: 100%;">Logout</button>
      </div>
    </div>
  </div>
</template>
