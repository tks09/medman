<script setup>
import { useUi } from '../stores'
import Header from './Header.vue'
const ui = useUi()
</script>

<template>
  <div class="stage">
    <div class="ambient"><div class="glow g1"></div><div class="glow g2"></div></div>
    <div class="device">
<div class="app">
        <Header />
        <slot />
      </div>
    </div>
  </div>
</template>
