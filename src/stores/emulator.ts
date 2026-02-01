import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Emulator {
  id: string
  type: 'ios' | 'android' | 'harmony'
  name: string
  deviceType: string
  osVersion: string
  status: 'running' | 'stopped'
  lastUsedAt?: number
}

export const useEmulatorStore = defineStore('emulator', () => {
  const emulators = ref<Emulator[]>([])
  const loading = ref(false)
  const currentType = ref<'ios' | 'android' | 'harmony'>('android')

  async function fetchEmulators(type: 'ios' | 'android' | 'harmony') {
    loading.value = true
    currentType.value = type
    try {
      let result: any[] = []
      if (type === 'ios') {
        result = await invoke('list_ios_simulators')
      } else if (type === 'android') {
        result = await invoke('list_android_emulators')
      } else if (type === 'harmony') {
        result = await invoke('list_harmony_emulators')
      }
      
      emulators.value = result.map(e => ({
        id: e.id,
        type,
        name: e.name,
        deviceType: e.device_type,
        osVersion: e.os_version,
        status: e.status as 'running' | 'stopped'
      }))
    } finally {
      loading.value = false
    }
  }

  async function startEmulator(id: string) {
    const type = currentType.value
    if (type === 'ios') {
      await invoke('start_ios_simulator', { id })
    } else if (type === 'android') {
      await invoke('start_android_emulator', { id })
    } else if (type === 'harmony') {
      await invoke('start_harmony_emulator', { id })
    }
  }

  async function stopEmulator(id: string) {
    const type = currentType.value
    if (type === 'ios') {
      await invoke('stop_ios_simulator', { id })
    } else if (type === 'android') {
      await invoke('stop_android_emulator', { id })
    } else if (type === 'harmony') {
      await invoke('stop_harmony_emulator', { id })
    }
  }

  async function deleteEmulator(id: string) {
    const type = currentType.value
    if (type === 'ios') {
      await invoke('delete_ios_simulator', { id })
    } else if (type === 'android') {
      await invoke('delete_android_emulator', { id })
    }
  }

  async function wipeData(id: string) {
    const type = currentType.value
    if (type === 'ios') {
      await invoke('wipe_ios_data', { id })
    } else if (type === 'android') {
      await invoke('wipe_android_data', { id })
    }
  }

  async function takeScreenshot(id: string) {
    const type = currentType.value
    if (type === 'ios') {
      await invoke('screenshot_ios', { id })
    } else if (type === 'android') {
      await invoke('screenshot_android', { id })
    } else if (type === 'harmony') {
      await invoke('screenshot_harmony', { id })
    }
  }

  return {
    emulators,
    loading,
    fetchEmulators,
    startEmulator,
    stopEmulator,
    deleteEmulator,
    wipeData,
    takeScreenshot
  }
})
