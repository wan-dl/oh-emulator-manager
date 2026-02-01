<template>
  <n-card class="emulator-card" hoverable>
    <div class="card-content">
      <div class="drag-handle">⋮⋮</div>
      <div class="emulator-icon">
        <n-icon size="48" :component="getIcon()" />
      </div>
      <div class="emulator-info">
        <div class="emulator-name">{{ emulator.name }}</div>
        <div class="emulator-details">{{ emulator.deviceType }} · {{ emulator.osVersion }}</div>
        <n-tag :type="emulator.status === 'running' ? 'success' : 'default'" size="small">
          {{ t(`status.${emulator.status}`) }}
        </n-tag>
      </div>
      <div class="emulator-actions">
        <n-button
          v-if="emulator.status === 'stopped'"
          type="primary"
          @click="$emit('start', emulator.id)"
        >
          {{ t('actions.start') }}
        </n-button>
        <n-button
          v-else
          type="error"
          @click="$emit('stop', emulator.id)"
        >
          {{ t('actions.stop') }}
        </n-button>
        <n-dropdown :options="dropdownOptions" @select="handleAction">
          <n-button>{{ t('actions.more') }} ▼</n-button>
        </n-dropdown>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NCard, NButton, NTag, NIcon, NDropdown } from 'naive-ui'
import { PhonePortrait, TabletPortrait, DesktopOutline } from '@vicons/ionicons5'
import { useI18n } from 'vue-i18n'
import type { Emulator } from '@/stores/emulator'

const props = defineProps<{
  emulator: Emulator
}>()

const emit = defineEmits<{
  start: [id: string]
  stop: [id: string]
  delete: [id: string]
  wipeData: [id: string]
  screenshot: [id: string]
  viewLogs: [id: string]
  copyId: [id: string]
}>()

const { t } = useI18n()

const getIcon = () => {
  if (props.emulator.type === 'ios') return PhonePortrait
  if (props.emulator.type === 'android') return TabletPortrait
  return DesktopOutline
}

const dropdownOptions = computed(() => [
  { label: t('actions.copyId'), key: 'copyId' },
  { label: t('actions.screenshot'), key: 'screenshot' },
  { label: t('actions.viewLogs'), key: 'viewLogs' },
  { label: t('actions.wipeData'), key: 'wipeData' },
  { label: t('actions.delete'), key: 'delete' }
])

const handleAction = (key: string) => {
  emit(key as any, props.emulator.id)
}
</script>

<style scoped>
.emulator-card {
  margin-bottom: 12px;
}

.card-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.drag-handle {
  cursor: grab;
  color: #999;
  font-size: 20px;
  user-select: none;
}

.emulator-icon {
  color: #666;
}

.emulator-info {
  flex: 1;
}

.emulator-name {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 4px;
}

.emulator-details {
  font-size: 14px;
  color: #666;
  margin-bottom: 8px;
}

.emulator-actions {
  display: flex;
  gap: 8px;
}
</style>
