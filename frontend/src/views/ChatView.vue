<script setup>
import { ref, reactive, computed, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { usePlans } from '../stores'
import BaseIcon from '../components/BaseIcon.vue'

const router = useRouter()
const plansStore = usePlans()
const { t } = useI18n()

const STEPS = computed(() => [
  { key: 'name', q: () => t('chat.steps.nameQ'), chips: t('chat.steps.nameChips'), ph: t('chat.steps.namePh') },
  { key: 'dose', q: (p) => t('chat.steps.doseQ', { name: p.name }), chips: t('chat.steps.doseChips'), ph: t('chat.steps.dosePh') },
  { key: 'focus', q: () => t('chat.steps.focusQ'), chips: t('chat.steps.focusChips'), multi: true, ph: t('chat.steps.focusPh') },
  { key: 'duration', q: () => t('chat.steps.durationQ'), chips: t('chat.steps.durationChips'), ph: t('chat.steps.durationPh') },
  { key: 'reminder', q: () => t('chat.steps.reminderQ'), chips: t('chat.steps.reminderChips'), ph: '' },
])

const ROWS = computed(() => [
  { key: 'med', icon: 'pill', label: t('chat.rows.med') },
  { key: 'focus', icon: 'target', label: t('chat.rows.focus') },
  { key: 'duration', icon: 'calendar', label: t('chat.rows.duration') },
  { key: 'reminder', icon: 'bell', label: t('chat.rows.reminder') },
])

const msgs = ref([{ from: 'ai', text: STEPS.value[0].q() }])
const step = ref(0)
const typing = ref(false)
const focusSel = ref([])
const input = ref('')
const sheet = ref(false)
const done = ref(false)
const draft = reactive({ name: null, dose: null, focus: [], duration: null, reminder: null })
const scroller = ref(null)

const cur = computed(() => (step.value < STEPS.value.length ? STEPS.value[step.value] : null))

function rowState(k) {
  if (k === 'med') return draft.dose ? 'done' : (draft.name ? 'active' : 'idle')
  if (k === 'focus') return draft.focus.length ? 'done' : (step.value === 2 ? 'active' : 'idle')
  if (k === 'duration') return draft.duration ? 'done' : (step.value === 3 ? 'active' : 'idle')
  if (k === 'reminder') return draft.reminder ? 'done' : (step.value === 4 ? 'active' : 'idle')
}
const pct = computed(() => Math.round((['med', 'focus', 'duration', 'reminder'].filter((k) => rowState(k) === 'done').length / 4) * 100))

function rowValue(k) {
  if (k === 'med') return draft.name ? `${draft.name}${draft.dose ? ' · ' + draft.dose : ''}` : '—'
  if (k === 'focus') return draft.focus.length ? draft.focus.join(', ') : '—'
  if (k === 'duration') return draft.duration || '—'
  return draft.reminder || '—'
}

async function scrollDown() { await nextTick(); const s = scroller.value; if (s) s.scrollTop = s.scrollHeight + 400 }

function aiSay(text, after) {
  typing.value = true
  setTimeout(() => { typing.value = false; msgs.value.push({ from: 'ai', text }); scrollDown(); after && after() }, 750)
}

function advance(nextStep, patch, meText) {
  msgs.value.push({ from: 'me', text: meText }); scrollDown()
  Object.assign(draft, patch)
  if (nextStep < STEPS.value.length) {
    aiSay(STEPS.value[nextStep].q(draft))
    step.value = nextStep
  } else {
    step.value = nextStep
    aiSay(t('chat.aiPlanReady'), () => { done.value = true })
    setTimeout(() => { sheet.value = true }, 1100)
  }
}

function answer(val) {
  const s = STEPS.value[step.value]
  if (s.key === 'name') advance(1, { name: val }, val)
  else if (s.key === 'dose') advance(2, { dose: val }, val)
  else if (s.key === 'duration') advance(4, { duration: val }, val)
  else if (s.key === 'reminder') advance(5, { reminder: val }, val)
}
function toggleFocus(f) { focusSel.value = focusSel.value.includes(f) ? focusSel.value.filter((x) => x !== f) : [...focusSel.value, f] }
function continueFocus() { if (focusSel.value.length) advance(3, { focus: [...focusSel.value] }, focusSel.value.join(', ')) }
function sendInput() {
  const v = input.value.trim(); if (!v) return; input.value = ''
  if (cur.value?.key === 'focus') { const nf = [...focusSel.value, v]; focusSel.value = nf; advance(3, { focus: nf }, nf.join(', ')) }
  else answer(v)
}

async function generate() {
  await plansStore.create({ name: draft.name, dose: draft.dose || 'As needed', focus: draft.focus, adherence: 0 })
  router.push('/plan')
}
</script>

<template>
  <div class="chat-root" style="flex:1;min-height:0;display:flex;flex-direction:column">
  <div class="row between center" style="padding:2px 18px 10px">
    <button class="chip" style="padding:10px 12px" @click="router.push('/')"><BaseIcon name="back" :size="18" /></button>
    <div class="row gap8 center"><div class="bav" style="width:26px;height:26px"><BaseIcon name="spark" :size="14" /></div><span class="strong">{{ $t('chat.newPlan') }}</span></div>
    <button class="chip" style="padding:10px 12px" @click="router.push('/')"><BaseIcon name="x" :size="18" /></button>
  </div>

  <div class="screen" ref="scroller" style="padding-bottom:200px">
    <div class="thread">
      <template v-for="(m, i) in msgs" :key="i">
        <div v-if="m.from === 'ai'" class="brow">
          <div class="bav"><BaseIcon name="spark" :size="15" /></div>
          <div class="bubble" style="white-space:pre-line">{{ m.text }}</div>
        </div>
        <div v-else><div class="bubble me">{{ m.text }}</div></div>
      </template>
      <div v-if="typing" class="brow">
        <div class="bav"><BaseIcon name="spark" :size="15" /></div>
        <div class="bubble"><span class="typing"><i></i><i></i><i></i></span></div>
      </div>
    </div>
  </div>

  <div class="composer">
    <button @click="sheet = true" class="glass strong"
      style="display:flex;align-items:center;gap:10px;border:1px solid var(--hair);border-radius:18px;padding:11px 15px;cursor:pointer;text-align:left">
      <div class="pico" style="width:30px;height:30px;border-radius:9px;color:var(--accent)"><BaseIcon name="pill" :size="16" /></div>
      <div class="col grow" style="gap:5px">
        <div class="row between"><span class="t-xs strong">{{ $t('chat.planSoFar') }}</span><span class="t-xs strong" style="color:var(--accent)">{{ pct }}%</span></div>
        <div class="bar" style="height:5px"><i :style="{ width: pct + '%' }"></i></div>
      </div>
      <div style="color:var(--muted);transform:rotate(-90deg)"><BaseIcon name="chevron" :size="16" /></div>
    </button>

    <div v-if="!done && cur" class="chips">
      <button v-for="c in cur.chips" :key="c" class="chip" :class="{ sel: cur.multi && focusSel.includes(c) }" style="flex:none"
        @click="cur.multi ? toggleFocus(c) : answer(c)">
        <BaseIcon v-if="cur.multi && focusSel.includes(c)" name="check" :size="14" />{{ c }}
      </button>
      <button v-if="cur.multi" class="chip" style="flex:none;color:var(--accent-ink);background:linear-gradient(180deg,color-mix(in oklab,var(--ai) 88%,#fff),var(--ai));border-color:transparent"
        :style="{ opacity: focusSel.length ? 1 : .5 }" @click="continueFocus">Continue <BaseIcon name="arrow" :size="14" /></button>
    </div>

    <div class="glass strong" style="display:flex;align-items:center;gap:8px;padding:8px 8px 8px 16px;border-radius:26px">
      <input v-model="input" @keydown.enter="sendInput"
        :placeholder="done ? $t('chat.planComplete') : (cur ? cur.ph : $t('chat.typeMessage'))"
        style="flex:1;border:none;background:transparent;outline:none;font-family:var(--font);font-size:15px;color:var(--ink)" />
      <button class="send" @click="sendInput"><BaseIcon name="send" :size="18" /></button>
    </div>
  </div>

  <div class="scrim" :class="{ show: sheet }" @click="sheet = false"></div>
  <div class="sheet" :class="{ open: sheet }" :style="{ '--peek': '0px', transform: sheet ? 'translateY(0)' : 'translateY(100%)' }">
    <div class="grab" @click="sheet = false"></div>
    <div class="row between center" style="margin-bottom:6px">
      <div class="h-md">{{ $t('chat.yourPlan') }}</div>
      <span class="tag">{{ pct }}{{ $t('chat.pctComplete') }}</span>
    </div>
    <div>
      <div v-for="r in ROWS" :key="r.key" class="planrow">
        <div class="pico" :class="{ done: rowState(r.key) === 'done', active: rowState(r.key) === 'active' }">
          <BaseIcon :name="rowState(r.key) === 'done' ? 'check' : r.icon" :size="18" />
        </div>
        <div class="col grow" style="gap:3px">
          <span class="t-xs strong" style="color:var(--muted)">{{ r.label.toUpperCase() }}</span>
          <span class="strong" :style="{ color: rowState(r.key) === 'idle' ? 'var(--muted)' : 'var(--ink)', fontSize: '14.5px' }">
            <span v-if="rowState(r.key) === 'active' && rowValue(r.key) === '—'" style="color:var(--accent)">{{ $t('chat.answering') }}</span>
            <span v-else>{{ rowValue(r.key) }}</span>
          </span>
        </div>
        <BaseIcon v-if="rowState(r.key) === 'done'" name="edit" :size="16" />
      </div>
    </div>
    <button class="btn primary block lg" style="margin-top:16px" :disabled="pct < 100" @click="generate">
      <template v-if="pct < 100">{{ $t('chat.keepChatting') }}</template>
      <template v-else>{{ $t('chat.generatePlan') }} <BaseIcon name="spark" :size="18" /></template>
    </button>
  </div>
  </div>
</template>
