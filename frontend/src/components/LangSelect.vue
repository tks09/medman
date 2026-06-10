<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useUi } from '../stores'
import BaseIcon from './BaseIcon.vue'

const ui = useUi()
const { locale } = useI18n()
const open = ref(false)
const langs = [['en', 'English'], ['de', 'Deutsch'], ['fr', 'Français']]

function pick(code) {
  ui.setLang(code)
  locale.value = code
  open.value = false
}
</script>

<template>
  <div style="position:relative;flex:none">
    <button class="chip" style="padding:9px 12px" @click="open = !open" :aria-expanded="open">
      <BaseIcon name="globe" :size="17" /> {{ ui.lang.toUpperCase() }}
      <span style="display:inline-flex;transition:transform .2s var(--ease);color:var(--muted)"
            :style="{ transform: open ? 'rotate(-90deg)' : 'rotate(90deg)' }"><BaseIcon name="chevron" :size="14" /></span>
    </button>
    <template v-if="open">
      <div @click="open = false" style="position:fixed;inset:0;z-index:90"></div>
      <div class="glass strong" role="listbox"
           style="position:absolute;top:calc(100% + 8px);left:0;z-index:91;min-width:158px;padding:6px;border-radius:16px">
        <button v-for="[code, name] in langs" :key="code" role="option" @click="pick(code)"
          :style="{ display:'flex', alignItems:'center', gap:'10px', width:'100%', border:'none', cursor:'pointer',
                    background: ui.lang === code ? 'color-mix(in oklab,var(--accent) 16%,transparent)' : 'transparent',
                    color:'var(--ink)', fontFamily:'var(--font)', fontWeight: ui.lang === code ? 700 : 600,
                    fontSize:'14px', padding:'10px 11px', borderRadius:'11px', textAlign:'left' }">
          <span style="font-weight:800;width:24px">{{ code.toUpperCase() }}</span>
          <span class="grow" style="color:var(--ink-2);font-weight:500">{{ name }}</span>
          <span v-if="ui.lang === code" style="color:var(--accent);display:inline-flex"><BaseIcon name="check" :size="16" /></span>
        </button>
      </div>
    </template>
  </div>
</template>
