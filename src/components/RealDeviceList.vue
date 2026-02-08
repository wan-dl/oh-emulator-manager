<template>
  <div class="real-device-list">
    <n-spin :show="loading" class="spin-container">
      <div v-if="devices.length === 0" class="empty-container">
        <n-empty :description="t('realDevice.noDevices') || '未检测到 USB 设备'" />
      </div>
      <div v-else class="device-items">
        <real-device-card
          v-for="(device, index) in filteredDevices"
          :key="device.serial"
          :class="{ 'stripe': index % 2 === 1 }"
          :device="device"
          @copy-id="$emit('copyId', $event)"
        />
      </div>
    </n-spin>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NSpin, NEmpty } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import RealDeviceCard from './RealDeviceCard.vue'
import type { RealDevice } from './RealDeviceCard.vue'

const { t } = useI18n()

const props = defineProps<{
  devices: RealDevice[]
  loading?: boolean
  searchText?: string
}>()

defineEmits<{
  copyId: [serial: string]
}>()

const filteredDevices = computed(() => {
  if (!props.searchText) return props.devices
  const text = props.searchText.toLowerCase()
  return props.devices.filter(d =>
    d.name.toLowerCase().includes(text) ||
    d.brand.toLowerCase().includes(text) ||
    d.serial.toLowerCase().includes(text)
  )
})
</script>

<style scoped>
.real-device-list {
  overflow-y: auto;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.spin-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.device-items {
  flex: 1;
}

.device-items :deep(.real-device-card.stripe) {
  background-color: rgba(0, 0, 0, 0.02);
}

.device-items :deep(.real-device-card.stripe:hover) {
  background-color: rgba(0, 0, 0, 0.04);
}

.empty-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 300px;
}
</style>
