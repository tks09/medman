<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useUi, useAuth } from '../stores'
import BaseIcon from '../components/BaseIcon.vue'
import LangSelect from '../components/LangSelect.vue'

const ui = useUi()
const auth = useAuth()
const router = useRouter()

const mode = ref('login')
const name = ref('')
const email = ref('')
const password = ref('')

async function submit() {
  if (mode.value === 'signup') await auth.signup({ name: name.value, email: email.value, password: password.value })
  else await auth.login({ email: email.value, password: password.value })
  router.push('/')
}
async function social(provider) {
  await auth.oauth(provider)
  router.push('/')
}
</script>

<template>
  <div class="screen flush col" style="padding:0 22px;justify-content:center">
    <div class="row between" style="position:absolute;top:54px;left:22px;right:22px">
      <LangSelect />
      <button class="chip" @click="ui.toggleTheme()" style="padding:9px 11px">
        <BaseIcon :name="ui.dark ? 'sun' : 'moon'" :size="18" />
      </button>
    </div>

    <div class="col center" style="gap:14px;margin-bottom:26px">
      <div style="width:64px;height:64px;border-radius:20px;display:grid;place-items:center;color:#fff;
        background:linear-gradient(150deg,var(--accent),var(--ai));
        box-shadow:inset 0 1px 0 rgba(255,255,255,.4),0 14px 30px -10px color-mix(in oklab,var(--accent) 60%,transparent)">
        <BaseIcon name="pill" :size="32" />
      </div>
      <div class="col center" style="gap:3px">
        <div class="h-lg">Medman</div>
        <div class="t-xs">{{ $t('login.tagline') }}</div>
      </div>
    </div>

    <div class="glass strong card" style="border-radius:var(--radius);padding:22px">
      <div class="h-md" style="margin-bottom:3px">{{ mode === 'signup' ? $t('login.createAccount') : $t('login.welcomeBack') }}</div>
      <div class="t-xs" style="margin-bottom:16px">
        {{ mode === 'signup' ? $t('login.startFirst') : $t('login.pickUpWhere') }}
      </div>
      <form class="stack" @submit.prevent="submit">
        <input v-if="mode === 'signup'" v-model="name" class="field" :placeholder="$t('login.fullName')" />
        <input v-model="email" class="field" type="email" :placeholder="$t('login.email')" />
        <input v-model="password" class="field" type="password" :placeholder="$t('login.password')" />
        <div v-if="mode === 'login'" class="row between" style="margin-top:4px">
          <span class="t-xs">{{ $t('login.rememberMe') }}</span>
          <span class="t-xs strong" style="color:var(--accent)">{{ $t('login.forgot') }}</span>
        </div>
        <button class="btn primary block lg" type="submit" style="margin-top:12px" :disabled="auth.loading">
          {{ auth.loading ? $t('login.pleaseWait') : (mode === 'signup' ? $t('login.createAccount') : $t('login.logIn')) }}
        </button>
      </form>

      <div class="row center" style="gap:12px;margin:16px 0;color:var(--muted)">
        <div class="grow" style="height:1px;background:var(--hair-edge)"></div>
        <span class="t-xs" style="white-space:nowrap">{{ $t('login.orContinueWith') }}</span>
        <div class="grow" style="height:1px;background:var(--hair-edge)"></div>
      </div>
      <div class="row gap10">
        <button class="btn grow" @click="social('apple')"><BaseIcon name="apple" :size="18" /> Apple</button>
        <button class="btn grow" @click="social('google')"><BaseIcon name="google" :size="18" /> Google</button>
      </div>
    </div>

    <div class="t-sm" style="text-align:center;margin-top:18px">
      {{ mode === 'signup' ? $t('login.alreadyHaveAccount') + ' ' : $t('login.newToMedman') + ' ' }}
      <span class="strong" style="color:var(--accent);cursor:pointer" @click="mode = mode === 'signup' ? 'login' : 'signup'">
        {{ mode === 'signup' ? $t('login.logInLink') : $t('login.createOne') }}
      </span>
    </div>
  </div>
</template>
