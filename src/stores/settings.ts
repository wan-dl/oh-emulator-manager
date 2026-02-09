import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  settingsValidators,
  type ValidatableSettingKey,
  type ValidationResult
} from '@/utils/validation'

interface Settings {
  language: 'zh-CN' | 'en-US'
  theme: 'light' | 'dark' | 'system'
  auto_start: boolean
  minimize_to_tray: boolean
  android_home: string
  deveco_home: string
  harmony_image_location: string
  harmony_emulator_location: string
  harmony_emulator_path: string
  harmony_hdc_path: string
  xcode_home: string
  screenshot_dir: string
  android_force_kill: boolean
}

export const useSettingsStore = defineStore('settings', () => {
  const language = ref<'zh-CN' | 'en-US'>('zh-CN')
  const theme = ref<'light' | 'dark' | 'system'>('system')
  const autoStart = ref(false)
  const minimizeToTray = ref(true)
  const androidHome = ref('')
  const devecoHome = ref('')
  const harmonyImageLocation = ref('')
  const harmonyEmulatorLocation = ref('')
  const harmonyEmulatorPath = ref('')
  const harmonyHdcPath = ref('')
  const xcodeHome = ref('')
  const screenshotDir = ref('')
  const androidForceKill = ref(false)

  // Validation errors state
  const validationErrors = ref<Record<string, string | undefined>>({})

  // Map frontend field names to validator keys
  const fieldToValidatorMap: Record<string, ValidatableSettingKey> = {
    androidHome: 'androidHome',
    xcodeHome: 'xcodeHome',
    devecoHome: 'devecoHome',
    harmonyEmulatorPath: 'harmonyEmulatorPath',
    harmonyHdcPath: 'harmonyHdcPath',
    harmonyImageLocation: 'harmonyImageLocation',
    harmonyEmulatorLocation: 'harmonyEmulatorLocation',
    screenshotDir: 'screenshotDir'
  }

  // Get field value by name
  function getFieldValue(field: string): string {
    const fieldMap: Record<string, () => string> = {
      androidHome: () => androidHome.value,
      xcodeHome: () => xcodeHome.value,
      devecoHome: () => devecoHome.value,
      harmonyEmulatorPath: () => harmonyEmulatorPath.value,
      harmonyHdcPath: () => harmonyHdcPath.value,
      harmonyImageLocation: () => harmonyImageLocation.value,
      harmonyEmulatorLocation: () => harmonyEmulatorLocation.value,
      screenshotDir: () => screenshotDir.value
    }
    return fieldMap[field]?.() ?? ''
  }

  /**
   * Validate a single field
   */
  async function validateField(field: string): Promise<ValidationResult> {
    const validatorKey = fieldToValidatorMap[field]
    if (!validatorKey) {
      return { valid: true }
    }

    const validator = settingsValidators[validatorKey]
    const value = getFieldValue(field)
    const result = await validator(value)

    if (result.valid) {
      validationErrors.value[field] = undefined
    } else {
      validationErrors.value[field] = result.message
    }

    return result
  }

  /**
   * Validate all fields
   */
  async function validateAll(): Promise<boolean> {
    let allValid = true

    for (const field of Object.keys(fieldToValidatorMap)) {
      const result = await validateField(field)
      if (!result.valid) {
        allValid = false
      }
    }

    return allValid
  }

  /**
   * Get validation error for a field
   */
  function getError(field: string): string | undefined {
    return validationErrors.value[field]
  }

  /**
   * Check if a field has error
   */
  function hasError(field: string): boolean {
    return !!validationErrors.value[field]
  }

  /**
   * Check if all fields are valid
   */
  const isValid = computed(() => {
    return Object.values(validationErrors.value).every((error) => !error)
  })

  /**
   * Clear all validation errors
   */
  function clearErrors() {
    validationErrors.value = {}
  }

  async function loadSettings() {
    try {
      const settings = (await invoke('get_settings')) as Settings
      language.value = settings.language
      theme.value = settings.theme
      autoStart.value = settings.auto_start
      minimizeToTray.value = settings.minimize_to_tray
      androidHome.value = settings.android_home
      devecoHome.value = settings.deveco_home
      harmonyImageLocation.value = settings.harmony_image_location
      harmonyEmulatorLocation.value = settings.harmony_emulator_location
      harmonyEmulatorPath.value = settings.harmony_emulator_path
      harmonyHdcPath.value = settings.harmony_hdc_path
      xcodeHome.value = settings.xcode_home
      screenshotDir.value = settings.screenshot_dir
      androidForceKill.value = settings.android_force_kill

      // Clear errors after loading
      clearErrors()
    } catch (error) {
      console.error('Failed to load settings:', error)
    }
  }

  async function saveSettings(): Promise<boolean> {
    // Validate all fields before saving
    const isValid = await validateAll()
    if (!isValid) {
      return false
    }

    await invoke('save_settings', {
      settings: {
        language: language.value,
        theme: theme.value,
        auto_start: autoStart.value,
        minimize_to_tray: minimizeToTray.value,
        android_home: androidHome.value,
        deveco_home: devecoHome.value,
        harmony_image_location: harmonyImageLocation.value,
        harmony_emulator_location: harmonyEmulatorLocation.value,
        harmony_emulator_path: harmonyEmulatorPath.value,
        harmony_hdc_path: harmonyHdcPath.value,
        xcode_home: xcodeHome.value,
        screenshot_dir: screenshotDir.value,
        android_force_kill: androidForceKill.value
      }
    })

    return true
  }

  return {
    // State
    language,
    theme,
    autoStart,
    minimizeToTray,
    androidHome,
    devecoHome,
    harmonyImageLocation,
    harmonyEmulatorLocation,
    harmonyEmulatorPath,
    harmonyHdcPath,
    xcodeHome,
    screenshotDir,
    androidForceKill,

    // Validation
    validationErrors,
    isValid,
    validateField,
    validateAll,
    getError,
    hasError,
    clearErrors,

    // Actions
    loadSettings,
    saveSettings
  }
})
