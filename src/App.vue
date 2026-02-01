<template>
  <n-config-provider :theme="theme" :locale="locale" :date-locale="dateLocale">
    <n-message-provider>
      <n-dialog-provider>
        <n-notification-provider>
          <router-view />
        </n-notification-provider>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NConfigProvider, NMessageProvider, NDialogProvider, NNotificationProvider, darkTheme, zhCN, dateZhCN, enUS, dateEnUS } from 'naive-ui'
import { useSettingsStore } from '@/stores/settings'

const settingsStore = useSettingsStore()

const theme = computed(() => {
  if (settingsStore.theme === 'dark') return darkTheme
  if (settingsStore.theme === 'light') return null
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? darkTheme : null
})

const locale = computed(() => settingsStore.language === 'zh-CN' ? zhCN : enUS)
const dateLocale = computed(() => settingsStore.language === 'zh-CN' ? dateZhCN : dateEnUS)
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

#app {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}
</style>
