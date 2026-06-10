<script setup>
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
const props = defineProps({ mood: String, labels: { type: Boolean, default: false }, size: { type: Number, default: 40 } })
const emit = defineEmits(['update:mood'])
const MOODS = [
  { k: 'great', c: '#2bd4a8', mouth: 'M9 18q5 5 12 0' },
  { k: 'good', c: '#7bd47b', mouth: 'M10 18q4 3 10 0' },
  { k: 'ok', c: '#f4c84a', mouth: 'M10 19h10' },
  { k: 'low', c: '#f59e7a', mouth: 'M10 20q4-3 10 0' },
  { k: 'bad', c: '#f47a8c', mouth: 'M9 21q5-5 12 0' },
]
</script>

<template>
  <div class="row between">
    <button v-for="m in MOODS" :key="m.k" @click="emit('update:mood', m.k)" :aria-label="t('mood.' + m.k)"
            class="col center" style="background:none;border:none;cursor:pointer;gap:6px;padding:0">
      <span :style="{ transform: mood === m.k ? 'scale(1.14)' : 'scale(1)', transition:'transform .2s var(--ease)',
                      boxShadow: mood === m.k ? '0 0 0 3px color-mix(in oklab,var(--accent) 50%,transparent)' : 'none',
                      borderRadius:'50%', lineHeight:0, display:'inline-flex' }">
        <svg :width="size" :height="size" viewBox="0 0 30 30">
          <circle cx="15" cy="15" r="14" :fill="m.c" opacity="0.92" />
          <circle cx="10.5" cy="12.5" r="1.7" fill="#0c2522" /><circle cx="19.5" cy="12.5" r="1.7" fill="#0c2522" />
          <path :d="m.mouth" stroke="#0c2522" stroke-width="1.8" fill="none" stroke-linecap="round" />
        </svg>
      </span>
      <span v-if="labels" class="t-xs" :style="{ color: mood === m.k ? 'var(--ink)' : 'var(--muted)', fontWeight: mood === m.k ? 700 : 500 }">{{ t('mood.' + m.k) }}</span>
    </button>
  </div>
</template>
