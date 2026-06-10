<script setup>
import { useAuth } from '../stores'
import { useRouter, RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'

const auth = useAuth()
const router = useRouter()
const { t, locale } = useI18n()

const logout = () => {
  auth.logout()
  router.push('/login')
}

const changeLanguage = (lang) => {
  locale.value = lang
  localStorage.setItem('locale', lang)
}
</script>

<template>
  <div class="glass strong" style="padding: 10px 18px; border-bottom: 1px solid var(--hair); position: relative; z-index: 40;">
    <div class="row between center">
      <div class="row gap12 center">
        <span class="h-md nowrap">{{ t('navbar.medicineManager') }}</span>
        <div v-if="auth.isAuthed" class="row gap10">
          <RouterLink to="/dashboard" class="t-sm strong" style="color: var(--ink-2); text-decoration: none;">{{ t('navbar.dashboard') }}</RouterLink>
          <RouterLink to="/chat" class="t-sm strong" style="color: var(--ink-2); text-decoration: none;">{{ t('navbar.generatePlan') }}</RouterLink>
        </div>
      </div>
      <div class="row gap10 center">
        <div class="seg" style="padding: 4px;">
          <button @click="changeLanguage('en')" :class="{ on: locale === 'en' }" style="font-size: 12px; padding: 4px 8px;">EN</button>
          <button @click="changeLanguage('de')" :class="{ on: locale === 'de' }" style="font-size: 12px; padding: 4px 8px;">DE</button>
          <button @click="changeLanguage('fr')" :class="{ on: locale === 'fr' }" style="font-size: 12px; padding: 4px 8px;">FR</button>
        </div>
        <template v-if="auth.isAuthed">
          <span class="t-sm" style="color: var(--ink-2);">{{ t('navbar.welcome') }}, {{ auth.user?.name }}</span>
          <button class="chip" @click="logout" style="padding: 8px 14px; font-size: 13px;">{{ t('navbar.logout') }}</button>
        </template>
        <template v-else>
          <RouterLink to="/login" class="t-sm strong" style="color: var(--ink-2); text-decoration: none;">{{ t('navbar.login') }}</RouterLink>
          <RouterLink to="/register" class="chip" style="padding: 8px 14px; font-size: 13px;">{{ t('navbar.register') }}</RouterLink>
        </template>
      </div>
    </div>
  </div>
</template>

