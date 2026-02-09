/**
 * Settings validation utilities
 */

import { invoke } from '@tauri-apps/api/core'

export interface ValidationResult {
  valid: boolean
  message?: string
}

export interface PathValidationOptions {
  required?: boolean
  mustExist?: boolean
  type?: 'file' | 'directory' | 'any'
  allowedExtensions?: string[]
}

/**
 * Check if a path exists using backend
 */
async function checkPathExists(path: string): Promise<boolean> {
  try {
    return await invoke<boolean>('check_path_exists', { path })
  } catch (error) {
    console.error('Error checking path existence:', error)
    return false
  }
}

/**
 * Validate a file/directory path
 */
export async function validatePath(
  value: string,
  options: PathValidationOptions = {}
): Promise<ValidationResult> {
  const { required = false, mustExist = false, allowedExtensions } = options

  // Check if required
  if (required && !value.trim()) {
    return { valid: false, message: 'validation.pathRequired' }
  }

  // Empty is valid if not required
  if (!value.trim()) {
    return { valid: true }
  }

  // Platform-specific detection
  const isWindows = navigator.platform.toLowerCase().includes('win')

  // Check for dangerous characters (command injection prevention)
  // Note: backslash is allowed on Windows as path separator
  const dangerousChars = isWindows ? /[;&|`$<>'"]/ : /[;&|`$<>'"]/
  if (dangerousChars.test(value)) {
    return { valid: false, message: 'validation.pathDangerousChars' }
  }

  // Check for path traversal attempts
  if (value.includes('..')) {
    return { valid: false, message: 'validation.pathTraversal' }
  }

  // Check file extension if specified
  if (allowedExtensions && allowedExtensions.length > 0) {
    const ext = value.split('.').pop()?.toLowerCase()
    if (!ext || !allowedExtensions.includes(ext)) {
      return {
        valid: false,
        message: 'validation.invalidExtension'
      }
    }
  }

  // Platform-specific path validation

  if (isWindows) {
    // Windows path validation
    // Must start with drive letter or UNC path
    const windowsPathRegex = /^([a-zA-Z]:[/\\]|\\\\)/
    if (!windowsPathRegex.test(value)) {
      return { valid: false, message: 'validation.invalidWindowsPath' }
    }

    // Check for reserved Windows names
    const reservedNames = /^(con|prn|aux|nul|com[1-9]|lpt[1-9])$/i
    const fileName = value.split(/[/\\]/).pop() || ''
    if (reservedNames.test(fileName.split('.')[0])) {
      return { valid: false, message: 'validation.reservedWindowsName' }
    }
  } else {
    // Unix path validation - must start with /
    if (!value.startsWith('/')) {
      return { valid: false, message: 'validation.invalidUnixPath' }
    }
  }

  // Check if path exists (if required)
  if (mustExist) {
    const exists = await checkPathExists(value)
    if (!exists) {
      return { valid: false, message: 'validation.pathNotExists' }
    }
  }

  return { valid: true }
}

/**
 * Validate Android SDK path
 */
export async function validateAndroidSdkPath(value: string): Promise<ValidationResult> {
  const pathResult = await validatePath(value, { mustExist: true })
  if (!pathResult.valid) {
    return pathResult
  }

  if (!value.trim()) {
    return { valid: true }
  }

  // Android SDK should contain specific directories
  // We can't check file existence from frontend, but we can validate format
  return { valid: true }
}

/**
 * Validate Xcode path
 */
export async function validateXcodePath(value: string): Promise<ValidationResult> {
  const pathResult = await validatePath(value, { mustExist: true })
  if (!pathResult.valid) {
    return pathResult
  }

  if (!value.trim()) {
    return { valid: true }
  }

  // Xcode path typically ends with .app or contains Xcode
  const normalized = value.toLowerCase()
  if (!normalized.includes('xcode') && !normalized.endsWith('.app')) {
    return { valid: false, message: 'validation.invalidXcodePath' }
  }

  return { valid: true }
}

/**
 * Validate DevEco Studio path
 */
export async function validateDevecoPath(value: string): Promise<ValidationResult> {
  const pathResult = await validatePath(value, { mustExist: true })
  if (!pathResult.valid) {
    return pathResult
  }

  return { valid: true }
}

/**
 * Validate executable file path
 */
export async function validateExecutablePath(value: string): Promise<ValidationResult> {
  const isWindows = navigator.platform.toLowerCase().includes('win')

  const pathResult = await validatePath(value, {
    mustExist: true,
    allowedExtensions: isWindows ? ['exe', 'bat', 'cmd'] : undefined
  })

  if (!pathResult.valid) {
    return pathResult
  }

  return { valid: true }
}

/**
 * Validate screenshot directory
 */
export async function validateScreenshotDir(value: string): Promise<ValidationResult> {
  return validatePath(value, { mustExist: true })
}

/**
 * All validation rules for settings
 */
export const settingsValidators = {
  androidHome: validateAndroidSdkPath,
  xcodeHome: validateXcodePath,
  devecoHome: validateDevecoPath,
  harmonyEmulatorPath: validateExecutablePath,
  harmonyHdcPath: validateExecutablePath,
  harmonyImageLocation: (value: string) => validatePath(value, { mustExist: true }),
  harmonyEmulatorLocation: (value: string) => validatePath(value, { mustExist: true }),
  screenshotDir: validateScreenshotDir
} as const

export type ValidatableSettingKey = keyof typeof settingsValidators
