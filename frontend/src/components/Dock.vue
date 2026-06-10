<script setup>
import { useRoute, useRouter } from 'vue-router'
import BaseIcon from './BaseIcon.vue'
const route = useRoute()
const router = useRouter()
const on = (name) => route.name === name
</script>

<template>
  <div class="dock">
    <button class="nav" :class="{ on: on('home') }" @click="router.push('/')" aria-label="Home"><BaseIcon name="home" :size="23" /></button>
    <button class="nav" :class="{ on: on('track') }" @click="router.push('/track')" aria-label="Tracking"><BaseIcon name="track" :size="23" /></button>
    <button class="fab" @click="router.push('/chat')" aria-label="New plan"><BaseIcon name="plus" :size="26" /></button>
    <button class="nav" :class="{ on: on('insights') }" @click="router.push('/insights')" aria-label="Insights"><BaseIcon name="insight" :size="23" /></button>
    <button class="nav" :class="{ on: on('plan') }" @click="router.push('/plan')" aria-label="Plan"><BaseIcon name="pill" :size="22" /></button>
  </div>
</template>
