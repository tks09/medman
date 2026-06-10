<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { usePlans } from '../stores'
import * as api from '../services/api'
import BaseIcon from '../components/BaseIcon.vue'
import Spark from '../components/Spark.vue'

const router = useRouter()
const plansStore = usePlans()
const series = ref([3, 2, 3, 4, 3, 4, 5, 4, 4, 5, 4, 5, 5])

const plan = computed(() => plansStore.active || { name: '—', dose: '', focus: [] })
const reviews = [
  'Sleep quality improved after moving dose to morning.',
  'Mild nausea on 2 days — both before breakfast.',
]

onMounted(async () => {
  if (!plansStore.loaded) await plansStore.load()
  if (plansStore.active) series.value = await api.getPlanSeries(plansStore.active.id)
})
</script>

<template>
  <div class="screen">
    <div class="row between center" style="margin-top:4px;margin-bottom:16px">
      <button class="chip" style="padding:10px 12px" @click="router.push('/')"><BaseIcon name="back" :size="18" /></button>
      <button class="chip" style="padding:10px 12px"><BaseIcon name="edit" :size="18" /></button>
    </div>
    <div class="row gap14 center">
      <div style="width:54px;height:54px;border-radius:18px;display:grid;place-items:center;color:#fff;background:linear-gradient(150deg,var(--accent),var(--ai))"><BaseIcon name="pill" :size="26" /></div>
      <div class="col"><div class="h-lg">{{ plan.name }}</div><div class="t-sm">{{ plan.dose }}</div></div>
    </div>
    <div class="row wrap gap8 mt16"><span v-for="f in plan.focus" :key="f" class="chip"><BaseIcon name="target" :size="15" /> {{ f }}</span></div>

    <div class="glass card mt20" style="border-radius:var(--radius-md)">
      <div class="row between center"><div class="strong">{{ $t('plan.wellbeingTrend') }}</div><span class="tag">13 days</span></div>
      <div class="mt16"><Spark :data="series" /></div>
      <div class="row between t-xs mt8"><span>May 28</span><span>Today</span></div>
    </div>

    <div class="row gap14 mt16">
      <div class="glass card grow" style="padding:16px"><div class="t-xs strong" style="color:var(--muted)">{{ $t('plan.dosesTaken') }}</div><div class="h-md mt8">26 / 30</div></div>
      <div class="glass card grow" style="padding:16px"><div class="t-xs strong" style="color:var(--muted)">{{ $t('plan.sideEffects') }}</div><div class="h-md mt8">3 {{ $t('plan.flags') }}</div></div>
    </div>

    <div class="h-md mt24" style="margin-bottom:12px">{{ $t('plan.recentReviews') }}</div>
    <div class="stack">
      <div v-for="(r, i) in reviews" :key="i" class="glass card" style="border-radius:var(--radius-md)">
        <div class="row gap10 center"><div class="pico" style="color:var(--ai)"><BaseIcon name="spark" :size="16" /></div><div class="t-xs strong" style="color:var(--muted)">JUN {{ 7 - i }} · {{ $t('plan.aiReview') }}</div></div>
        <div class="t-sm mt8">{{ r }}</div>
      </div>
    </div>
    <button class="btn primary block lg mt20" @click="router.push('/chat')">{{ $t('plan.askAssistant') }} <BaseIcon name="chat" :size="18" /></button>
  </div>
</template>
