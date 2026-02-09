<template>
  <div class="settings-view">
    <div class="header">
      <div class="header-left">
        <n-button text @click="router.back()">
          <template #icon>
            <n-icon :component="ArrowBack" />
          </template>
        </n-button>
        <h3>{{ t('settings.title') }}</h3>
      </div>
      <div class="header-center">
        <div class="tabs-header">
          <div
            v-for="tab in tabs"
            :key="tab.key"
            :class="['tab-item', { active: activeTab === tab.key }]"
            @click="activeTab = tab.key"
          >
            {{ tab.label }}
          </div>
        </div>
      </div>
      <div class="header-right">
        <!-- 占位，保持布局平衡 -->
      </div>
    </div>

    <div class="settings-content">
      <div class="content">
        <!-- 通用设置 -->
        <div v-if="activeTab === 'general'" class="tab-content">
          <n-form label-placement="left" label-width="160">
            <n-form-item :label="t('settings.language')">
              <div class="button-group">
                <button
                  v-for="option in languageOptions"
                  :key="option.value"
                  :class="['option-button', { active: settingsStore.language === option.value }]"
                  @click="settingsStore.language = option.value; handleAutoSave()"
                >
                  {{ option.label }}
                </button>
              </div>
            </n-form-item>
            <n-form-item :label="t('settings.theme')">
              <div class="button-group">
                <button
                  v-for="option in themeOptions"
                  :key="option.value"
                  :class="['option-button', { active: settingsStore.theme === option.value }]"
                  @click="settingsStore.theme = option.value; handleAutoSave()"
                >
                  {{ option.label }}
                </button>
              </div>
            </n-form-item>
            <n-form-item
              :label="t('settings.screenshotDir')"
              :validation-status="settingsStore.hasError('screenshotDir') ? 'error' : undefined"
              :feedback="settingsStore.hasError('screenshotDir') ? t(settingsStore.getError('screenshotDir')!) : undefined"
            >
              <n-input-group>
                <n-input
                  v-model:value="settingsStore.screenshotDir"
                  :status="settingsStore.hasError('screenshotDir') ? 'error' : undefined"
                  @blur="handleFieldBlur('screenshotDir')"
                  @input="handleFieldInput('screenshotDir')"
                />
                <n-button @click="selectScreenshotDir">
                  {{ t('settings.selectFolder') }}
                </n-button>
              </n-input-group>
            </n-form-item>
            <n-form-item :label="t('settings.autoStart')">
              <n-switch v-model:value="settingsStore.autoStart" @update:value="handleAutoSave" />
            </n-form-item>
            <n-form-item :label="t('settings.minimizeToTray')">
              <n-switch v-model:value="settingsStore.minimizeToTray" @update:value="handleAutoSave" />
            </n-form-item>
          </n-form>
        </div>

        <!-- Android 设置 -->
        <div v-if="activeTab === 'android'" class="tab-content">
          <n-form label-placement="left" label-width="160">
            <n-form-item
              :label="t('settings.androidHome')"
              :validation-status="settingsStore.hasError('androidHome') ? 'error' : undefined"
              :feedback="settingsStore.hasError('androidHome') ? t(settingsStore.getError('androidHome')!) : undefined"
            >
              <n-input-group>
                <n-input
                  v-model:value="settingsStore.androidHome"
                  :status="settingsStore.hasError('androidHome') ? 'error' : undefined"
                  @blur="handleFieldBlur('androidHome')"
                  @input="handleFieldInput('androidHome')"
                />
                <n-button @click="selectAndroidHome">
                  {{ t('settings.selectFolder') }}
                </n-button>
              </n-input-group>
            </n-form-item>
            <n-form-item :label="t('settings.androidForceKill')">
              <n-checkbox v-model:checked="settingsStore.androidForceKill" @update:checked="handleAutoSave">
                {{ t('settings.androidForceKillDesc') }}
              </n-checkbox>
            </n-form-item>
          </n-form>
        </div>

        <!-- iOS 设置 -->
        <div v-if="activeTab === 'ios'" class="tab-content">
          <n-form label-placement="left" label-width="160">
            <n-form-item
              :label="t('settings.xcodeHome')"
              :validation-status="settingsStore.hasError('xcodeHome') ? 'error' : undefined"
              :feedback="settingsStore.hasError('xcodeHome') ? t(settingsStore.getError('xcodeHome')!) : undefined"
            >
              <n-input-group>
                <n-input
                  v-model:value="settingsStore.xcodeHome"
                  :status="settingsStore.hasError('xcodeHome') ? 'error' : undefined"
                  @blur="handleFieldBlur('xcodeHome')"
                  @input="handleFieldInput('xcodeHome')"
                />
                <n-button @click="selectXcodeHome">
                  {{ t('settings.selectFolder') }}
                </n-button>
              </n-input-group>
            </n-form-item>
          </n-form>
        </div>

        <!-- 鸿蒙设置 -->
        <div v-if="activeTab === 'harmony'" class="tab-content">
          <n-form label-placement="left" label-width="200">
            <n-form-item
              :label="t('settings.devecoHome')"
              :validation-status="settingsStore.hasError('devecoHome') ? 'error' : undefined"
              :feedback="settingsStore.hasError('devecoHome') ? t(settingsStore.getError('devecoHome')!) : undefined"
            >
              <n-input-group>
                <n-input
                  v-model:value="settingsStore.devecoHome"
                  :status="settingsStore.hasError('devecoHome') ? 'error' : undefined"
                  @blur="handleFieldBlur('devecoHome')"
                  @input="handleFieldInput('devecoHome')"
                />
                <n-button @click="selectDevecoHome">
                  {{ t('settings.selectFolder') }}
                </n-button>
              </n-input-group>
            </n-form-item>
            <n-form-item
              :label="t('settings.harmonyEmulatorPath')"
              :validation-status="settingsStore.hasError('harmonyEmulatorPath') ? 'error' : undefined"
              :feedback="settingsStore.hasError('harmonyEmulatorPath') ? t(settingsStore.getError('harmonyEmulatorPath')!) : undefined"
            >
              <n-input-group>
                <n-input
                  v-model:value="settingsStore.harmonyEmulatorPath"
                  :status="settingsStore.hasError('harmonyEmulatorPath') ? 'error' : undefined"
                  :placeholder="t('settings.harmonyEmulatorPathPlaceholder')"
                  @blur="handleFieldBlur('harmonyEmulatorPath')"
                  @input="handleFieldInput('harmonyEmulatorPath')"
                />
                <n-button @click="selectHarmonyEmulatorPath">
                  {{ t('settings.selectFile') }}
                </n-button>
              </n-input-group>
            </n-form-item>
            <n-form-item
              :label="t('settings.harmonyHdcPath')"
              :validation-status="settingsStore.hasError('harmonyHdcPath') ? 'error' : undefined"
              :feedback="settingsStore.hasError('harmonyHdcPath') ? t(settingsStore.getError('harmonyHdcPath')!) : undefined"
            >
              <n-input-group>
                <n-input
                  v-model:value="settingsStore.harmonyHdcPath"
                  :status="settingsStore.hasError('harmonyHdcPath') ? 'error' : undefined"
                  :placeholder="t('settings.harmonyHdcPathPlaceholder')"
                  @blur="handleFieldBlur('harmonyHdcPath')"
                  @input="handleFieldInput('harmonyHdcPath')"
                />
                <n-button @click="selectHarmonyHdcPath">
                  {{ t('settings.selectFile') }}
                </n-button>
              </n-input-group>
            </n-form-item>
            <n-form-item
              :label="t('settings.harmonyImageLocation')"
              :validation-status="settingsStore.hasError('harmonyImageLocation') ? 'error' : undefined"
              :feedback="settingsStore.hasError('harmonyImageLocation') ? t(settingsStore.getError('harmonyImageLocation')!) : undefined"
            >
              <n-input-group>
                <n-input
                  v-model:value="settingsStore.harmonyImageLocation"
                  :status="settingsStore.hasError('harmonyImageLocation') ? 'error' : undefined"
                  @blur="handleFieldBlur('harmonyImageLocation')"
                  @input="handleFieldInput('harmonyImageLocation')"
                />
                <n-button @click="selectHarmonyImageLocation">
                  {{ t('settings.selectFolder') }}
                </n-button>
              </n-input-group>
            </n-form-item>
            <n-form-item
              :label="t('settings.harmonyEmulatorLocation')"
              :validation-status="settingsStore.hasError('harmonyEmulatorLocation') ? 'error' : undefined"
              :feedback="settingsStore.hasError('harmonyEmulatorLocation') ? t(settingsStore.getError('harmonyEmulatorLocation')!) : undefined"
            >
              <n-input-group>
                <n-input
                  v-model:value="settingsStore.harmonyEmulatorLocation"
                  :status="settingsStore.hasError('harmonyEmulatorLocation') ? 'error' : undefined"
                  @blur="handleFieldBlur('harmonyEmulatorLocation')"
                  @input="handleFieldInput('harmonyEmulatorLocation')"
                />
                <n-button @click="selectHarmonyEmulatorLocation">
                  {{ t('settings.selectFolder') }}
                </n-button>
              </n-input-group>
            </n-form-item>
          </n-form>
        </div>

        <!-- 关于 -->
        <div v-if="activeTab === 'about'" class="tab-content">
          <div class="about-content">
            <h3>{{ t('app.title') }}</h3>
            <p>{{ t('app.version') }}: 1.0.0</p>
            <p>{{ t('about.description') }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { NForm, NFormItem, NSwitch, NCheckbox, NInput, NInputGroup, NButton, NIcon, useMessage } from 'naive-ui'
import { ArrowBack } from '@vicons/ionicons5'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { useSettingsStore } from '@/stores/settings'
import { open } from '@tauri-apps/plugin-dialog'

const { t } = useI18n()
const router = useRouter()
const message = useMessage()
const settingsStore = useSettingsStore()

const activeTab = ref('general')

const tabs = computed(() => [
  { key: 'general', label: t('settings.general') },
  { key: 'android', label: 'Android' },
  { key: 'ios', label: 'iOS' },
  { key: 'harmony', label: t('settings.harmony') },
  { key: 'about', label: t('settings.about') }
])

const languageOptions: Array<{label: string, value: 'zh-CN' | 'en-US'}> = [
  { label: '中文', value: 'zh-CN' },
  { label: 'English', value: 'en-US' }
]

const themeOptions = computed<Array<{label: string, value: 'light' | 'dark' | 'system'}>>(() => [
  { label: t('theme.light'), value: 'light' },
  { label: t('theme.dark'), value: 'dark' },
  { label: t('theme.system'), value: 'system' }
])

// Validate field on blur
const handleFieldBlur = async (field: string) => {
  await settingsStore.validateField(field)
  await handleAutoSave()
}

// Clear error on input (will re-validate on blur)
const handleFieldInput = (field: string) => {
  // Clear error immediately when user starts typing
  if (settingsStore.hasError(field)) {
    settingsStore.validationErrors[field] = undefined
  }
}

const selectXcodeHome = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      canCreateDirectories: true,
      title: 'Select Xcode Path'
    })
    if (selected) {
      settingsStore.xcodeHome = selected as string
      await settingsStore.validateField('xcodeHome')
      await handleAutoSave()
    }
  } catch (error) {
    console.error('Error selecting folder:', error)
    message.error(t('validation.selectFolderError'))
  }
}

const selectScreenshotDir = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      canCreateDirectories: true,
      title: t('settings.selectScreenshotDir')
    })
    if (selected) {
      settingsStore.screenshotDir = selected as string
      await settingsStore.validateField('screenshotDir')
      await handleAutoSave()
    }
  } catch (error) {
    console.error('Error selecting folder:', error)
    message.error(t('validation.selectFolderError'))
  }
}

const selectAndroidHome = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      canCreateDirectories: true,
      title: 'Select Android SDK Path'
    })
    if (selected) {
      settingsStore.androidHome = selected as string
      await settingsStore.validateField('androidHome')
      await handleAutoSave()
    }
  } catch (error) {
    console.error('Error selecting folder:', error)
    message.error(t('validation.selectFolderError'))
  }
}

const selectDevecoHome = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      canCreateDirectories: true,
      title: 'Select DevEco Studio Path'
    })
    if (selected) {
      settingsStore.devecoHome = selected as string
      await settingsStore.validateField('devecoHome')
      await handleAutoSave()
    }
  } catch (error) {
    console.error('Error selecting folder:', error)
    message.error(t('validation.selectFolderError'))
  }
}

const selectHarmonyImageLocation = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      canCreateDirectories: true,
      title: t('settings.harmonyImageLocation')
    })
    if (selected) {
      settingsStore.harmonyImageLocation = selected as string
      await settingsStore.validateField('harmonyImageLocation')
      await handleAutoSave()
    }
  } catch (error) {
    console.error('Error selecting folder:', error)
    message.error(t('validation.selectFolderError'))
  }
}

const selectHarmonyEmulatorLocation = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      canCreateDirectories: true,
      title: t('settings.harmonyEmulatorLocation')
    })
    if (selected) {
      settingsStore.harmonyEmulatorLocation = selected as string
      await settingsStore.validateField('harmonyEmulatorLocation')
      await handleAutoSave()
    }
  } catch (error) {
    console.error('Error selecting folder:', error)
    message.error(t('validation.selectFolderError'))
  }
}

const selectHarmonyEmulatorPath = async () => {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      title: t('settings.harmonyEmulatorPath')
    })
    if (selected) {
      settingsStore.harmonyEmulatorPath = selected as string
      await settingsStore.validateField('harmonyEmulatorPath')
      await handleAutoSave()
    }
  } catch (error) {
    console.error('Error selecting file:', error)
    message.error(t('validation.selectFileError'))
  }
}

const selectHarmonyHdcPath = async () => {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      title: t('settings.harmonyHdcPath')
    })
    if (selected) {
      settingsStore.harmonyHdcPath = selected as string
      await settingsStore.validateField('harmonyHdcPath')
      await handleAutoSave()
    }
  } catch (error) {
    console.error('Error selecting file:', error)
    message.error(t('validation.selectFileError'))
  }
}

const handleAutoSave = async () => {
  try {
    const success = await settingsStore.saveSettings()
    if (!success) {
      message.warning(t('validation.fixErrorsBeforeSave'))
    }
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
  justify-content: space-between;
  padding: 16px 24px;
  border-bottom: 1px solid #e0e0e0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
  flex: 1;
}

.header-left h3 {
  font-size: 20px;
  font-weight: 600;
  margin: 0;
}

.header-center {
  display: flex;
  justify-content: center;
  flex: 1;
}

.header-right {
  flex: 1;
}

.settings-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.tabs-header {
  display: inline-flex;
  gap: 4px;
  padding: 3px;
  background: #f0f0f0;
  border-radius: 20px;
}

.tab-item {
  padding: 6px 20px;
  cursor: pointer;
  border-radius: 16px;
  transition: all 0.2s;
  color: #666;
  background: transparent;
  font-size: 13px;
  border: none;
  white-space: nowrap;
}

.tab-item:hover:not(.active) {
  background: rgba(0, 0, 0, 0.05);
}

.tab-item.active {
  background: white;
  color: #2080f0;
  font-weight: 500;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.content {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
  width: 100%;
  max-width: 800px;
  margin-top: 16px;
}

.tab-content {
  max-width: 600px;
  margin: 0 auto;
}

.button-group {
  display: inline-flex;
  gap: 4px;
  padding: 4px;
  background: #f0f0f0;
  border-radius: 20px;
}

.option-button {
  padding: 6px 20px;
  border: none;
  background: transparent;
  border-radius: 16px;
  cursor: pointer;
  transition: all 0.2s;
  color: #666;
  font-size: 14px;
  white-space: nowrap;
  outline: none;
}

.option-button:hover:not(.active) {
  background: rgba(0, 0, 0, 0.05);
}

.option-button.active {
  background: white;
  color: #2080f0;
  font-weight: 500;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.about-content {
  text-align: center;
  padding: 40px 0;
}

.about-content h3 {
  font-size: 24px;
  margin-bottom: 16px;
}

.about-content p {
  margin: 8px 0;
  color: #666;
}

.actions {
  display: flex;
  gap: 12px;
  margin-top: 32px;
  padding-top: 24px;
  border-top: 1px solid #e0e0e0;
}

/* Naive UI 组件蓝色主题 */
:deep(.n-switch) {
  border-radius: 18px !important;
}

:deep(.n-switch *) {
  border-radius: 18px !important;
}

:deep(.n-switch__rail) {
  border-radius: 18px !important;
}

:deep(.n-switch.n-switch--active) {
  background-color: #2080f0 !important;
  border-radius: 18px !important;
}

:deep(.n-switch.n-switch--active *) {
  border-radius: 18px !important;
}

:deep(.n-switch.n-switch--active .n-switch__rail) {
  border-radius: 18px !important;
  background-color: #2080f0 !important;
}

:deep(.n-switch__button) {
  border-radius: 50% !important;
}

:deep(.n-checkbox.n-checkbox--checked .n-checkbox-box) {
  background-color: #2080f0 !important;
  border-color: #2080f0 !important;
}

:deep(.n-button--primary-type) {
  background-color: #2080f0 !important;
  border-color: #2080f0 !important;
}

:deep(.n-button:not(.n-button--disabled):hover) {
  background-color: #4098fc !important;
  border-color: #4098fc !important;
}

/* Validation error styles */
:deep(.n-form-item-feedback-wrapper) {
  min-height: 22px;
}

:deep(.n-form-item--error .n-input .n-input__border) {
  border-color: #d03050 !important;
}

:deep(.n-form-item--error .n-input:hover .n-input__border) {
  border-color: #d03050 !important;
}

:deep(.n-form-item--error .n-input.n-input--focus .n-input__border) {
  border-color: #d03050 !important;
  box-shadow: 0 0 0 2px rgba(208, 48, 80, 0.2) !important;
}
</style>
