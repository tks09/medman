<script setup>
import { onMounted } from 'vue'
import { usePlans } from '../stores'
import BaseIcon from '../components/BaseIcon.vue'
import Spark from '../components/Spark.vue'

const plansStore = usePlans()
onMounted(() => { if (!plansStore.loaded) plansStore.load() })
</script>

<template>
  <div class="screen">
    <div class="h-xl" style="margin-top:6px">{{ $t('insights.title') }}</div>
    <div class="t-sm" style="margin-bottom:18px">{{ $t('insights.subtitle', { days: 13 }) }}</div>

    <div class="glass strong card" style="border-radius:var(--radius)">
      <div class="row between center"><div class="strong">{{ $t('insights.thisWeek') }}</div><span class="tag">Wellbeing +18%</span></div>
      <div class="mt16"><Spark :data="[2, 3, 3, 2, 4, 3, 4, 5, 4, 5]" /></div>
    </div>

    <div class="stack mt16">
      <div v-for="ins in plansStore.insights" :key="ins.id" class="glass card" style="border-radius:var(--radius-md)">
        <div class="row gap10 center">
          <div class="pico" :style="{ color: ins.tone === 'ai' ? 'var(--ai)' : 'var(--accent)' }"><BaseIcon :name="ins.icon" :size="18" /></div>
          <div class="strong grow">{{ ins.title }}</div>
          <BaseIcon name="chevron" :size="16" />
        </div>
        <div class="t-sm mt8">{{ ins.body }}</div>
      </div>
    </div>
  </div>
</template>
