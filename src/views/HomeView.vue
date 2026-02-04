<template>
  <div class="home-view">
    <!-- 第一列：平台选择 -->
    <div v-if="!sidebarCollapsed" class="platform-sidebar">
      <div class="sidebar-header">
        <h3>{{ t('app.title') }}</h3>
        <n-button text size="small" @click="sidebarCollapsed = true" class="collapse-btn">
          <img src="@/assets/silder.svg" class="slider-icon" />
        </n-button>
      </div>
      <div class="platform-list">
        <div 
          v-if="isMacOS"
          :class="['platform-item', { active: activeTab === 'ios' }]"
          @click="handleTabChange('ios')"
        >
          <img src="@/assets/iOS.svg" class="platform-icon" />
          <span>{{ t('tabs.ios') }}</span>
        </div>
        <div 
          :class="['platform-item', { active: activeTab === 'android' }]"
          @click="handleTabChange('android')"
        >
          <img src="@/assets/android.svg" class="platform-icon" />
          <span>{{ t('tabs.android') }}</span>
        </div>
        <div 
          :class="['platform-item', { active: activeTab === 'harmony' }]"
          @click="handleTabChange('harmony')"
        >
          <img src="@/assets/phone.svg" class="platform-icon" />
          <span>{{ t('tabs.harmony') }}</span>
        </div>
      </div>
      <div class="sidebar-footer">
        <n-button text @click="router.push('/settings')" class="settings-button">
          <img src="@/assets/settings.svg" class="settings-icon" />
          {{ t('actions.settings') }}
        </n-button>
      </div>
    </div>

    <!-- 第二列：设备列表 -->
    <div class="device-panel">
      <div class="device-header">
        <div 
          class="device-header-left" 
          :class="{ 'sidebar-collapsed': sidebarCollapsed }"
          :style="{ marginLeft: (sidebarCollapsed && isMacOS) ? '70px' : '0' }"
        >
          
          <n-button v-if="sidebarCollapsed" text size="small" @click="sidebarCollapsed = false" class="expand-btn">
            <img src="@/assets/silder.svg" class="slider-icon slider-icon-expand" />
          </n-button>
          <h4>{{ t('tabs.' + activeTab) }}</h4>
        </div>
        <div class="device-actions">
          <n-input
            v-model:value="searchText"
            :placeholder="t('actions.search')"
            clearable
            size="small"
            style="width: 180px"
          />
          <n-button @click="handleRefresh" size="small" quaternary circle>
            <template #icon>
              <n-icon :component="Refresh" />
            </template>
          </n-button>
          <n-button v-if="consoleCollapsed" @click="consoleCollapsed = false" size="small" quaternary circle>
            <img src="@/assets/console.svg" class="console-icon" />
          </n-button>
        </div>
      </div>
      <div class="device-content">
        <emulator-list
          :emulators="filteredEmulators"
          :loading="emulatorStore.loading"
          :search-text="searchText"
          :starting-emulators="startingEmulators"
          :stopping-emulators="stoppingEmulators"
          @start="handleStart"
          @stop="handleStop"
          @delete="handleDelete"
          @wipe-data="handleWipeData"
          @screenshot="handleScreenshot"
          @view-logs="handleViewLogs"
          @copy-id="handleCopyId"
        />
      </div>
    </div>

    <!-- 拖动分隔条 -->
    <div 
      v-if="!consoleCollapsed"
      class="resize-handle"
      @mousedown="startResize"
    ></div>

    <!-- 第三列：控制台 -->
    <transition name="console-slide">
      <div v-if="!consoleCollapsed" class="console-panel" :style="{ width: consolePanelWidth + 'px' }">
        <div class="console-header">
          <div class="console-header-left">
            <n-button text size="small" @click="consoleCollapsed = true" class="collapse-btn">
              <img src="@/assets/silder.svg" class="slider-icon" />
            </n-button>
            <span class="console-title">控制台</span>
          </div>
          <div class="console-header-actions">
            <n-button text size="small" @click="clearConsole">清空</n-button>
          </div>
        </div>
        <div class="console-body">
          <div class="console-content" ref="consoleRef">
            <div v-if="consoleTab === 'app'" class="console-logs">
              <div v-for="(log, index) in consoleLogs" :key="index" :class="['console-log', log.type]">
                <span class="console-time">{{ log.time }}</span>
                <span class="console-message">{{ log.message }}</span>
                <div v-if="log.path" class="console-path-container">
                  <span class="console-path" @click="openScreenshot(log.path)">
                    {{ log.path }}
                  </span>
                  <n-button 
                    text 
                    size="tiny" 
                    @click="copyPathToClipboard(log.path)"
                    class="copy-path-btn"
                    title="复制路径"
                  >
                    <img src="@/assets/copy.svg" class="btn-icon" />
                  </n-button>
                  <n-button 
                    text 
                    size="tiny" 
                    @click="copyImageToClipboard(log.path)"
                    class="copy-image-btn"
                    title="复制图片"
                  >
                    <img src="@/assets/image.svg" class="btn-icon" />
                  </n-button>
                </div>
              </div>
            </div>
            <div v-else-if="consoleTab === 'device'" class="console-logs">
              <div v-for="(log, index) in deviceLogs" :key="index" class="console-log device">
                <span class="console-time">{{ log.time }}</span>
                <span class="console-message">{{ log.message }}</span>
              </div>
              <div v-if="deviceLogs.length === 0" class="console-empty">
                <span>暂无设备日志</span>
                <span class="console-hint">启动模拟器后将显示 logcat 日志</span>
              </div>
            </div>
          </div>
          <div class="console-tabs">
            <div 
              :class="['console-tab', { active: consoleTab === 'app' }]"
              @click="consoleTab = 'app'"
              title="程序输出"
            >
              <img src="@/assets/app-log.svg" class="tab-icon" />
            </div>
            <div 
              :class="['console-tab', { active: consoleTab === 'device' }]"
              @click="handleDeviceLogTab"
              title="设备日志"
            >
              <img src="@/assets/device-log.svg" class="tab-icon" />
            </div>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, nextTick, watch } from 'vue'
import { NButton, NInput, NIcon, useMessage, useDialog } from 'naive-ui'
import { Refresh } from '@vicons/ionicons5'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { useEmulatorStore } from '@/stores/emulator'
import EmulatorList from '@/components/EmulatorList.vue'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'

const { t } = useI18n()
const router = useRouter()
const message = useMessage()
const dialog = useDialog()
const emulatorStore = useEmulatorStore()

const isMacOS = ref(false)
const activeTab = ref(localStorage.getItem('activeTab') || 'android')
const searchText = ref('')
const consoleLogs = ref<Array<{type: string, message: string, time: string, path?: string}>>([])
const deviceLogs = ref<Array<{message: string, time: string}>>([])
const consoleRef = ref<HTMLElement>()
const consoleCollapsed = ref(true)
const consoleTab = ref<'app' | 'device'>('app')
const consolePanelWidth = ref(320)
const isResizing = ref(false)
const startingEmulators = ref<Set<string>>(new Set())
const stoppingEmulators = ref<Set<string>>(new Set())
const sidebarCollapsed = ref(false)
const logcatProcess = ref<any>(null)

const filteredEmulators = computed(() => {
  const emulators = emulatorStore.emulators.filter(emulator => emulator.type === activeTab.value)
  return emulators.sort((a, b) => {
    if (a.status === 'running' && b.status !== 'running') return -1
    if (a.status !== 'running' && b.status === 'running') return 1
    return 0
  })
})

const addConsoleLog = (type: string, message: string, path?: string) => {
  const time = new Date().toLocaleTimeString()
  consoleLogs.value.push({ type, message, time, path })
  // 只在错误时自动展开控制台
  if (type === 'error') {
    consoleCollapsed.value = false
  }
  nextTick(() => {
    if (consoleRef.value) {
      consoleRef.value.scrollTop = consoleRef.value.scrollHeight
    }
  })
}

const clearConsole = () => {
  if (consoleTab.value === 'app') {
    consoleLogs.value = []
  } else {
    deviceLogs.value = []
  }
}

const handleDeviceLogTab = async () => {
  consoleTab.value = 'device'
  // 如果有运行中的模拟器，开始收集日志
  const runningEmulator = filteredEmulators.value.find(e => e.status === 'running')
  if (runningEmulator && !logcatProcess.value) {
    await startLogcat(runningEmulator.id)
  }
}

const startLogcat = async (deviceId: string) => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    // 启动 logcat 监听
    await invoke('start_logcat', { deviceId })
    
    // 定期获取日志
    const fetchLogs = async () => {
      try {
        const logs: string[] = await invoke('get_logcat_logs', { deviceId })
        const time = new Date().toLocaleTimeString()
        logs.forEach(log => {
          if (log.trim()) {
            deviceLogs.value.push({ message: log, time })
          }
        })
        
        // 限制日志数量
        if (deviceLogs.value.length > 1000) {
          deviceLogs.value = deviceLogs.value.slice(-1000)
        }
        
        nextTick(() => {
          if (consoleRef.value && consoleTab.value === 'device') {
            consoleRef.value.scrollTop = consoleRef.value.scrollHeight
          }
        })
      } catch (error) {
        console.error('Failed to fetch logcat:', error)
      }
    }
    
    // 每秒获取一次日志
    logcatProcess.value = setInterval(fetchLogs, 1000)
  } catch (error) {
    console.error('Failed to start logcat:', error)
    addConsoleLog('error', `无法启动设备日志: ${error}`)
  }
}

const stopLogcat = async () => {
  if (logcatProcess.value) {
    clearInterval(logcatProcess.value)
    logcatProcess.value = null
  }
  
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('stop_logcat')
  } catch (error) {
    console.error('Failed to stop logcat:', error)
  }
}

onMounted(async () => {
  const userAgent = navigator.userAgent.toLowerCase()
  isMacOS.value = userAgent.includes('mac')
  if (!isMacOS.value && activeTab.value === 'ios') {
    activeTab.value = 'android'
  }
  await handleRefresh()
  window.addEventListener('focus', handleRefresh)
  
  // 禁用触摸板滑动导航
  const preventNavigation = (e: WheelEvent) => {
    // 检测是否是水平滑动
    if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) {
      e.preventDefault()
    }
  }
  
  window.addEventListener('wheel', preventNavigation, { passive: false })
  
  // 清理函数
  onUnmounted(() => {
    window.removeEventListener('wheel', preventNavigation)
  })
})

onUnmounted(() => {
  window.removeEventListener('focus', handleRefresh)
  stopLogcat()
})

watch(consoleCollapsed, async (collapsed) => {
  try {
    const appWindow = getCurrentWindow()
    const currentSize = await appWindow.innerSize()
    
    if (collapsed) {
      await appWindow.setSize(new LogicalSize(828, currentSize.height))
    } else {
      await appWindow.setSize(new LogicalSize(828 + consolePanelWidth.value, currentSize.height))
    }
  } catch (error) {
    console.error('Failed to resize window:', error)
  }
})

// 监听控制台宽度变化，同步调整窗口大小
watch(consolePanelWidth, async (newWidth) => {
  if (!consoleCollapsed.value) {
    try {
      const appWindow = getCurrentWindow()
      const currentSize = await appWindow.innerSize()
      await appWindow.setSize(new LogicalSize(828 + newWidth, currentSize.height))
    } catch (error) {
      console.error('Failed to resize window:', error)
    }
  }
})

const handleTabChange = async (tab: string) => {
  activeTab.value = tab
  localStorage.setItem('activeTab', tab)
  await emulatorStore.fetchEmulators(tab as any)
}

const handleRefresh = async () => {
  await emulatorStore.fetchEmulators(activeTab.value as any)
}

const handleStart = async (id: string) => {
  try {
    startingEmulators.value.add(id)
    addConsoleLog('info', `正在启动模拟器: ${id}`)
    await emulatorStore.startEmulator(id)
    addConsoleLog('success', `模拟器启动成功: ${id}`)
    message.success(t('messages.startSuccess'))
    
    // 启动后轮询检查状态，确保UI更新
    let retries = 0
    const maxRetries = 30 // 增加到30次，每次2秒，总共60秒
    const checkStatus = async () => {
      await handleRefresh()
      const emulator = filteredEmulators.value.find(e => e.id === id || e.name === id)
      
      if (emulator?.status === 'running') {
        // 状态已更新为运行中
        startingEmulators.value.delete(id)
        addConsoleLog('success', `模拟器 ${id} 已完全启动`)
        return
      }
      
      retries++
      if (retries < maxRetries) {
        // 继续轮询，每2秒检查一次
        if (retries % 5 === 0) {
          // 每10秒输出一次进度
          addConsoleLog('info', `等待模拟器启动... (${retries * 2}秒)`)
        }
        setTimeout(checkStatus, 2000)
      } else {
        // 超时，停止轮询
        startingEmulators.value.delete(id)
        addConsoleLog('warning', `模拟器 ${id} 启动超时，但进程可能仍在启动中`)
      }
    }
    
    // 延迟2秒后开始检查
    setTimeout(checkStatus, 2000)
  } catch (error) {
    const errorMsg = typeof error === 'string' ? error : (error instanceof Error ? error.message : JSON.stringify(error))
    addConsoleLog('error', `模拟器启动失败: ${id} - ${errorMsg}`)
    console.error('Start emulator error:', error)
    message.error(errorMsg)
    startingEmulators.value.delete(id)
  }
}

const handleStop = async (id: string) => {
  try {
    stoppingEmulators.value.add(id)
    await emulatorStore.stopEmulator(id)
    message.success(t('messages.stopSuccess'))
    
    // 停止后轮询检查状态，确保UI更新
    let retries = 0
    const maxRetries = 10
    const checkStatus = async () => {
      await handleRefresh()
      const emulator = filteredEmulators.value.find(e => e.id === id || e.name === id)
      
      if (emulator?.status === 'stopped' || !emulator) {
        // 状态已更新为停止或模拟器不存在
        stoppingEmulators.value.delete(id)
        return
      }
      
      retries++
      if (retries < maxRetries) {
        // 继续轮询
        setTimeout(checkStatus, 1000)
      } else {
        // 超时，停止轮询
        stoppingEmulators.value.delete(id)
      }
    }
    
    // 延迟1秒后开始检查
    setTimeout(checkStatus, 1000)
  } catch (error) {
    const errorMsg = typeof error === 'string' ? error : (error instanceof Error ? error.message : JSON.stringify(error))
    addConsoleLog('error', `模拟器关闭失败: ${id} - ${errorMsg}`)
    console.error('Stop emulator error:', error)
    message.error(errorMsg)
    stoppingEmulators.value.delete(id)
  }
}

const handleDelete = async (id: string) => {
  dialog.warning({
    title: t('dialogs.deleteTitle'),
    content: t('dialogs.deleteContent', { id }),
    positiveText: t('actions.delete'),
    negativeText: t('settings.cancel'),
    onPositiveClick: async () => {
      try {
        await emulatorStore.deleteEmulator(id)
        message.success(t('messages.deleteSuccess'))
        await handleRefresh()
      } catch (error) {
        const errorMsg = typeof error === 'string' ? error : (error instanceof Error ? error.message : JSON.stringify(error))
        message.error(errorMsg)
      }
    }
  })
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
    const path = await emulatorStore.takeScreenshot(id)
    message.success(t('messages.screenshotSaved', { path }))
    // 将截图路径输出到控制台，标记为可点击的链接
    addConsoleLog('screenshot', `截图已保存`, path)
    consoleCollapsed.value = false
  } catch (error) {
    const errorMsg = typeof error === 'string' ? error : (error instanceof Error ? error.message : JSON.stringify(error))
    message.error(errorMsg)
    addConsoleLog('error', `截图失败: ${errorMsg}`)
  }
}

const handleViewLogs = (id: string) => {
  consoleCollapsed.value = false
  addConsoleLog('info', `查看模拟器日志: ${id}`)
}

const handleCopyId = (id: string) => {
  navigator.clipboard.writeText(id)
  message.success(t('messages.copySuccess'))
}

const openScreenshot = async (path: string) => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('open_file', { path })
  } catch (error) {
    console.error('Failed to open screenshot:', error)
    // 如果打开失败，尝试复制路径到剪贴板
    try {
      await navigator.clipboard.writeText(path)
      message.info('无法打开图片，路径已复制到剪贴板')
    } catch {
      message.error('无法打开图片')
    }
  }
}

const copyPathToClipboard = async (path: string) => {
  try {
    await navigator.clipboard.writeText(path)
    message.success('路径已复制到剪贴板')
  } catch (error) {
    console.error('Failed to copy path:', error)
    message.error('复制失败')
  }
}

const copyImageToClipboard = async (path: string) => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('copy_image_to_clipboard', { path })
    message.success('图片已复制到剪贴板')
  } catch (error) {
    console.error('Failed to copy image:', error)
    const errorMsg = typeof error === 'string' ? error : (error instanceof Error ? error.message : '复制失败')
    message.error(errorMsg)
  }
}

const startResize = (e: MouseEvent) => {
  isResizing.value = true
  const startX = e.clientX
  const startWidth = consolePanelWidth.value

  const handleMouseMove = (e: MouseEvent) => {
    if (!isResizing.value) return
    
    // 计算新宽度（从右边拖动，所以是减法）
    const deltaX = startX - e.clientX
    const newWidth = startWidth + deltaX
    
    // 限制最小和最大宽度
    if (newWidth >= 280 && newWidth <= 600) {
      consolePanelWidth.value = newWidth
    }
  }

  const handleMouseUp = () => {
    isResizing.value = false
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('mouseup', handleMouseUp)
    document.body.style.cursor = ''
    document.body.style.userSelect = ''
  }

  document.addEventListener('mousemove', handleMouseMove)
  document.addEventListener('mouseup', handleMouseUp)
  document.body.style.cursor = 'ew-resize'
  document.body.style.userSelect = 'none'
}
</script>

<style scoped src="./HomeView.css"></style>
