<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { usePlans } from '../stores'
import BaseIcon from '../components/BaseIcon.vue'
import MoodFaces from '../components/MoodFaces.vue'

const { t, locale } = useI18n()
const router = useRouter()
const plansStore = usePlans()
const sel = ref([])
const note = ref('')
const saving = ref(false)

const effects = computed(() => t('track.effects'))

function tog(e) { sel.value = sel.value.includes(e) ? sel.value.filter((x) => x !== e) : [...sel.value, e] }

async function save() {
  saving.value = true
  try { await plansStore.logCheckin({ sideEffects: sel.value, note: note.value }); router.push('/') }
  finally { saving.value = false }
}

const dateStr = computed(() => {
  const localeCode = locale.value === 'de' ? 'de-DE' : locale.value === 'fr' ? 'fr-FR' : 'en-US'
  const plan = plansStore.active
  const dateLabel = new Date().toLocaleDateString(localeCode, { weekday: 'long', month: 'long', day: 'numeric' })
  return plan ? `${dateLabel} · ${plan.name} ${plan.dose}` : dateLabel
})
</script>

<template>
  <div class="screen">
    <div class="h-xl" style="margin-top:6px">{{ $t('track.dailyCheckin') }}</div>
    <div class="t-sm" style="margin-bottom:18px">{{ dateStr }}</div>

    <div class="glass strong card" style="border-radius:var(--radius)">
      <div class="strong">{{ $t('track.howDoYouFeel') }}</div>
      <div style="margin-top:16px"><MoodFaces :mood="plansStore.mood" @update:mood="plansStore.setMood" :labels="true" :size="38" /></div>
    </div>

    <div class="glass card mt16" style="border-radius:var(--radius-md)">
      <div class="strong">{{ $t('track.sideEffectsToday') }}</div>
      <div class="row wrap gap8 mt16">
        <button v-for="e in effects" :key="e" class="chip" :class="{ sel: sel.includes(e) }" @click="tog(e)">{{ e }}</button>
      </div>
    </div>

    <div class="glass card mt16" style="border-radius:var(--radius-md)">
      <div class="strong">{{ $t('track.anythingToAdd') }}</div>
      <textarea v-model="note" class="field mt16" rows="3" :placeholder="$t('track.notePlaceholder')" style="resize:none"></textarea>
    </div>

    <button class="btn primary block lg mt20" :disabled="saving" @click="save">{{ saving ? $t('track.saving') : $t('track.saveCheckin') }} <BaseIcon name="check" :size="18" /></button>
  </div>
</template>
