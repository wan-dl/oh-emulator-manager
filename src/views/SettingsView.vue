<template>
  <div class="settings-view">
    <div class="header">
      <n-button text @click="router.back()">
        <template #icon>
          <n-icon :component="ArrowBack" />
        </template>
      </n-button>
      <h1>{{ t('settings.title') }}</h1>
    </div>

    <div class="settings-content">
      <n-card :title="t('settings.general')">
        <n-form label-placement="left" label-width="120">
          <n-form-item :label="t('settings.language')">
            <n-select
              v-model:value="settingsStore.language"
              :options="languageOptions"
            />
          </n-form-item>
          <n-form-item :label="t('settings.theme')">
            <n-select
              v-model:value="settingsStore.theme"
              :options="themeOptions"
            />
          </n-form-item>
          <n-form-item :label="t('settings.autoStart')">
            <n-switch v-model:value="settingsStore.autoStart" />
          </n-form-item>
          <n-form-item :label="t('settings.minimizeToTray')">
            <n-switch v-model:value="settingsStore.minimizeToTray" />
          </n-form-item>
          <n-form-item :label="t('settings.closeToMinimize')">
            <n-switch v-model:value="settingsStore.closeToMinimize" />
          </n-form-item>
        </n-form>
      </n-card>

      <n-card :title="t('settings.paths')" style="margin-top: 16px">
        <n-form label-placement="left" label-width="120">
          <n-form-item :label="t('settings.androidHome')">
            <n-input-group>
              <n-input v-model:value="settingsStore.androidHome" readonly />
              <n-button @click="selectAndroidHome">
                {{ t('settings.selectFolder') }}
              </n-button>
            </n-input-group>
          </n-form-item>
          <n-form-item :label="t('settings.devecoHome')">
            <n-input-group>
              <n-input v-model:value="settingsStore.devecoHome" readonly />
              <n-button @click="selectDevecoHome">
                {{ t('settings.selectFolder') }}
              </n-button>
            </n-input-group>
          </n-form-item>
        </n-form>
      </n-card>

      <div class="actions">
        <n-button type="primary" @click="handleSave">
          {{ t('settings.save') }}
        </n-button>
        <n-button @click="router.back()">
          {{ t('settings.cancel') }}
        </n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NCard, NForm, NFormItem, NSelect, NSwitch, NInput, NInputGroup, NButton, NIcon, useMessage } from 'naive-ui'
import { ArrowBack } from '@vicons/ionicons5'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { useSettingsStore } from '@/stores/settings'
import { open } from '@tauri-apps/plugin-dialog'

const { t } = useI18n()
const router = useRouter()
const message = useMessage()
const settingsStore = useSettingsStore()

const languageOptions = [
  { label: '中文', value: 'zh-CN' },
  { label: 'English', value: 'en-US' }
]

const themeOptions = computed(() => [
  { label: t('theme.light'), value: 'light' },
  { label: t('theme.dark'), value: 'dark' },
  { label: t('theme.system'), value: 'system' }
])

const selectAndroidHome = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select Android SDK Path'
  })
  if (selected) {
    settingsStore.androidHome = selected as string
  }
}

const selectDevecoHome = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select DevEco Studio Path'
  })
  if (selected) {
    settingsStore.devecoHome = selected as string
  }
}

const handleSave = async () => {
  try {
    await settingsStore.saveSettings()
    message.success(t('messages.settingsSaved'))
    router.back()
  } catch (error) {
    message.error(t('messages.error'))
  }
}
</script>

<style scoped>
.settings-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 24px;
  border-bottom: 1px solid #e0e0e0;
}

.header h1 {
  font-size: 20px;
  font-weight: 600;
}

.settings-content {
  padding: 24px;
  overflow-y: auto;
}

.actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}
</style>
