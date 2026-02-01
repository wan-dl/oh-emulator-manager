<template>
  <div class="home-view">
    <div class="header">
      <div class="logo">
        <h1>{{ t('app.title') }}</h1>
      </div>
      <div class="header-actions">
        <n-button text @click="router.push('/settings')">
          <template #icon>
            <n-icon :component="Settings" />
          </template>
          {{ t('actions.settings') }}
        </n-button>
      </div>
    </div>

    <div class="toolbar">
      <n-tabs v-model:value="activeTab" type="line" @update:value="handleTabChange">
        <n-tab-pane v-if="isMacOS" name="ios" :tab="t('tabs.ios')" />
        <n-tab-pane name="android" :tab="t('tabs.android')" />
        <n-tab-pane name="harmony" :tab="t('tabs.harmony')" />
      </n-tabs>
      <div class="toolbar-actions">
        <n-input
          v-model:value="searchText"
          :placeholder="t('actions.search')"
          clearable
          style="width: 200px"
        />
        <n-button @click="handleRefresh">
          <template #icon>
            <n-icon :component="Refresh" />
          </template>
          {{ t('actions.refresh') }}
        </n-button>
      </div>
    </div>

    <emulator-list
      :emulators="emulatorStore.emulators"
      :loading="emulatorStore.loading"
      :search-text="searchText"
      @start="handleStart"
      @stop="handleStop"
      @delete="handleDelete"
      @wipe-data="handleWipeData"
      @screenshot="handleScreenshot"
      @view-logs="handleViewLogs"
      @copy-id="handleCopyId"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NTabs, NTabPane, NButton, NInput, NIcon, useMessage } from 'naive-ui'
import { Settings, Refresh } from '@vicons/ionicons5'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { useEmulatorStore } from '@/stores/emulator'
import EmulatorList from '@/components/EmulatorList.vue'

const { t } = useI18n()
const router = useRouter()
const message = useMessage()
const emulatorStore = useEmulatorStore()

const isMacOS = ref(false)
const activeTab = ref('android')
const searchText = ref('')

onMounted(async () => {
  // Check platform
  isMacOS.value = navigator.platform.toLowerCase().includes('mac')
  if (!isMacOS.value && activeTab.value === 'ios') {
    activeTab.value = 'android'
  }
  await handleRefresh()
})

const handleTabChange = async (tab: string) => {
  await emulatorStore.fetchEmulators(tab as any)
}

const handleRefresh = async () => {
  await emulatorStore.fetchEmulators(activeTab.value as any)
}

const handleStart = async (id: string) => {
  try {
    await emulatorStore.startEmulator(id)
    message.success(t('messages.startSuccess'))
  } catch (error) {
    message.error(t('messages.error'))
  }
}

const handleStop = async (id: string) => {
  try {
    await emulatorStore.stopEmulator(id)
    message.success(t('messages.stopSuccess'))
  } catch (error) {
    message.error(t('messages.error'))
  }
}

const handleDelete = async (id: string) => {
  try {
    await emulatorStore.deleteEmulator(id)
    message.success(t('messages.deleteSuccess'))
    await handleRefresh()
  } catch (error) {
    message.error(t('messages.error'))
  }
}

const handleWipeData = async (id: string) => {
  try {
    await emulatorStore.wipeData(id)
    message.success(t('messages.wipeDataSuccess'))
  } catch (error) {
    message.error(t('messages.error'))
  }
}

const handleScreenshot = async (id: string) => {
  try {
    await emulatorStore.takeScreenshot(id)
    message.success(t('messages.screenshotSuccess'))
  } catch (error) {
    message.error(t('messages.error'))
  }
}

const handleViewLogs = (id: string) => {
  // TODO: Open log viewer dialog
  console.log('View logs:', id)
}

const handleCopyId = (id: string) => {
  navigator.clipboard.writeText(id)
  message.success(t('messages.copySuccess'))
}
</script>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-bottom: 1px solid #e0e0e0;
}

.logo h1 {
  font-size: 20px;
  font-weight: 600;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 24px;
  border-bottom: 1px solid #e0e0e0;
}

.toolbar-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}
</style>
