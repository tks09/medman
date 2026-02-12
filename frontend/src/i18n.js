import { createI18n } from 'vue-i18n'

// Import translation files
import enTranslations from './locales/en.json'
import deTranslations from './locales/de.json'
import frTranslations from './locales/fr.json'

const i18n = createI18n({
  legacy: false, // you must set this to false for Composition API
  locale: 'en', // set default locale
  fallbackLocale: 'en', // set fallback locale
  messages: {
    en: enTranslations,
    de: deTranslations,
    fr: frTranslations
  }
})

export default i18n