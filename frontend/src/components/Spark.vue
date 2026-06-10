<script setup>
const props = defineProps({ data: { type: Array, default: () => [] }, w: { type: Number, default: 290 }, h: { type: Number, default: 70 }, fill: { type: Boolean, default: true } })
const max = Math.max(...props.data), min = Math.min(...props.data), rng = (max - min) || 1
const pts = props.data.map((v, i) => [(i / (props.data.length - 1)) * props.w, props.h - 6 - ((v - min) / rng) * (props.h - 16)])
const d = pts.map((p, i) => (i ? 'L' : 'M') + p[0].toFixed(1) + ' ' + p[1].toFixed(1)).join(' ')
const area = d + ` L ${props.w} ${props.h} L 0 ${props.h} Z`
const last = pts[pts.length - 1]
</script>

<template>
  <svg width="100%" :viewBox="`0 0 ${w} ${h}`" preserveAspectRatio="none" style="display:block">
    <defs><linearGradient id="sg" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="var(--accent)" stop-opacity="0.32"/><stop offset="1" stop-color="var(--accent)" stop-opacity="0"/></linearGradient></defs>
    <path v-if="fill" :d="area" fill="url(#sg)"/>
    <path :d="d" fill="none" stroke="var(--accent)" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"/>
    <circle :cx="last[0]" :cy="last[1]" r="3.5" fill="var(--accent)"/>
  </svg>
</template>
