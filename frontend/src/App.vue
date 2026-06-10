<script setup>
import { onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useUi, useAuth } from './stores'
import DeviceShell from './components/DeviceShell.vue'
import Dock from './components/Dock.vue'

const ui = useUi()
const auth = useAuth()
const route = useRoute()
const { locale } = useI18n()

onMounted(() => {
  ui.apply()
  locale.value = ui.lang.toLowerCase()
})

const showDock = computed(() => auth.isAuthed && ['home', 'track', 'insights', 'plan'].includes(route.name))
</script>

<template>
  <DeviceShell>
    <router-view v-slot="{ Component }">
      <transition name="screen" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
    <Dock v-if="showDock" />
  </DeviceShell>
</template>

<style>
.screen-enter-active, .screen-leave-active { transition: opacity .3s var(--ease), transform .3s var(--ease); }
.screen-enter-from { opacity: 0; transform: translateY(8px) scale(.99); }
.screen-leave-to { opacity: 0; }
@media (prefers-reduced-motion: reduce) {
  .screen-enter-active, .screen-leave-active { transition: none; }
}
</style>
