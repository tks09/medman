<script setup>
import { onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { usePlans } from '../stores'
import BaseIcon from '../components/BaseIcon.vue'
import MoodFaces from '../components/MoodFaces.vue'

const { t } = useI18n()
const plans = usePlans()
const router = useRouter()

onMounted(() => { if (!plans.loaded) plans.load() })

const active = computed(() => plans.active)

const greeting = computed(() => {
  const h = new Date().getHours()
  if (h < 12) return t('home.goodMorning')
  if (h < 17) return t('home.goodAfternoon')
  return t('home.goodEvening')
})
</script>

<template>
  <div class="screen">
    <div style="margin-top:6px;margin-bottom:18px">
      <div class="h-xl">{{ greeting }}</div>
    </div>

    <!-- check-in hero -->
    <div v-if="active" class="glass strong card" style="border-radius:var(--radius)">
      <div class="row between center">
        <div class="eyebrow">{{ $t('home.todaysCheckin') }}</div>
        <span class="tag">{{ $t('home.twoMin') }}</span>
      </div>
      <div class="h-md" style="margin-top:8px">{{ $t('home.howFeelingOn', { name: active.name }) }}</div>
      <div style="margin-top:16px">
        <MoodFaces :mood="plans.mood" @update:mood="plans.setMood" :size="40" />
      </div>
      <button class="btn primary block" style="margin-top:18px" @click="router.push('/track')">{{ $t('home.logCheckin') }} <BaseIcon name="arrow" :size="18" /></button>
    </div>

    <!-- stats -->
    <div class="row gap14 mt16">
      <div class="glass card grow" style="padding:16px">
        <div class="row gap8 center" style="color:var(--accent)"><BaseIcon name="flame" :size="20" /><span class="t-xs strong" style="color:var(--muted)">{{ $t('home.streak') }}</span></div>
        <div class="h-lg mt8">12 <span style="font-size:14px;font-weight:600;color:var(--muted)">{{ $t('home.days') }}</span></div>
      </div>
      <div class="glass card grow" style="padding:16px">
        <div class="row gap8 center" style="color:var(--accent)"><BaseIcon name="check" :size="20" /><span class="t-xs strong" style="color:var(--muted)">{{ $t('home.adherence') }}</span></div>
        <div class="h-lg mt8">86<span style="font-size:16px;font-weight:700;color:var(--muted)">%</span></div>
      </div>
    </div>

    <!-- plans -->
    <div class="row between center mt24" style="margin-bottom:12px">
      <div class="h-md nowrap">{{ $t('home.yourPlans') }}</div>
      <span class="t-xs strong nowrap" style="color:var(--accent);cursor:pointer" @click="router.push('/chat')">{{ $t('home.new') }}</span>
    </div>
    <div class="hscroll">
      <div v-for="p in plans.plans" :key="p.id" class="glass card" style="width:208px;border-radius:var(--radius-md);cursor:pointer" @click="router.push('/plan/' + p.id)">
        <div class="row between center">
          <div class="pico done"><BaseIcon name="pill" :size="18" /></div>
          <BaseIcon name="chevron" :size="18" />
        </div>
        <div class="h-md mt12">{{ p.name }}</div>
        <div class="t-xs">{{ p.dose }}</div>
        <div class="row wrap gap6 mt12"><span v-for="f in p.focus" :key="f" class="tag">{{ f }}</span></div>
        <div class="bar mt16"><i :style="{ width: p.adherence + '%' }"></i></div>
      </div>
      <div class="glass card col center" style="width:130px;border-radius:var(--radius-md);justify-content:center;gap:10px;cursor:pointer;border-style:dashed" @click="router.push('/chat')">
        <div class="pico"><BaseIcon name="plus" :size="20" /></div>
        <div class="t-sm strong" style="text-align:center">{{ $t('home.newPlan') }}</div>
      </div>
    </div>

    <!-- insights -->
    <div class="row between center mt24" style="margin-bottom:12px">
      <div class="h-md nowrap">{{ $t('home.aiInsights') }}</div>
      <span class="t-xs strong nowrap" style="color:var(--accent);cursor:pointer" @click="router.push('/insights')">{{ $t('home.seeAll') }}</span>
    </div>
    <div class="stack">
      <div v-for="ins in plans.insights.slice(0, 2)" :key="ins.id" class="glass card" style="border-radius:var(--radius-md)">
        <div class="row gap10 center">
          <div class="pico" :style="{ color: ins.tone === 'ai' ? 'var(--ai)' : 'var(--accent)' }"><BaseIcon :name="ins.icon" :size="18" /></div>
          <div class="strong">{{ ins.title }}</div>
        </div>
        <div class="t-sm mt8">{{ ins.body }}</div>
      </div>
    </div>
  </div>
</template>
