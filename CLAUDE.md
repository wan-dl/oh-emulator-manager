# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/claude-code) when working with code in this repository.

## Project Overview

OH Emulator Manager is a cross-platform desktop application for managing iOS, Android, and HarmonyOS emulators through a unified visual interface. Built for software testers who frequently need to launch and manage emulators.

## Tech Stack

- **Framework**: Tauri 2.x (Rust backend + native WebView)
- **Frontend**: Vue 3 + TypeScript (Composition API)
- **UI Library**: Naive UI or Element Plus
- **State Management**: Pinia
- **Build Tool**: Vite
- **Database**: SQLite (rusqlite)
- **Styling**: UnoCSS or Tailwind CSS

## Project Structure

```
oh-emulator-manager/
├── src/                      # Frontend source code
│   ├── assets/               # Static assets
│   ├── components/           # Reusable components
│   ├── views/                # Page views
│   ├── stores/               # Pinia stores
│   ├── i18n/                 # Internationalization (zh-CN, en-US)
│   ├── App.vue
│   └── main.ts
├── src-tauri/                # Tauri/Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/         # Tauri commands (ios.rs, android.rs, harmony.rs)
│   │   ├── db/               # Database modules
│   │   ├── watcher/          # File system watcher
│   │   └── tray.rs           # System tray
│   ├── Cargo.toml
│   └── tauri.conf.json
├── _docs/                    # Project documentation
└── package.json
```

## Common Commands

```bash
# Install dependencies
pnpm install

# Development
pnpm tauri dev

# Build for production
pnpm tauri build

# Run frontend only
pnpm dev

# Type check
pnpm type-check

# Lint
pnpm lint
```

## Platform-Specific Notes

### iOS Simulator (macOS only)
- Requires Xcode and Command Line Tools
- Uses `xcrun simctl` for all operations
- Simulator directory: `~/Library/Developer/CoreSimulator/Devices/`

### Android Emulator (Windows/macOS)
- Requires Android SDK (ANDROID_HOME environment variable)
- Uses `emulator` and `adb` commands
- AVD directory: `~/.android/avd/`

### HarmonyOS NEXT Emulator (Windows/macOS)
- Requires DevEco Studio 5.x+
- Uses `hdctool` and `hdc` commands
- Only supports HarmonyOS NEXT (not legacy HarmonyOS 4.x)

## Key Features to Implement

1. **Emulator List**: Display all available emulators with status
2. **Emulator Operations**: Start, stop, delete, wipe data, screenshot, view logs
3. **Settings**: Language, theme, auto-start, minimize to tray, SDK paths
4. **System Tray**: Quick access to recent emulators (Windows tray / macOS menu bar)
5. **File Watching**: Auto-refresh emulator list on directory changes

## Database Schema

### emulators table
- id (TEXT PRIMARY KEY)
- type (TEXT): ios/android/harmony
- name (TEXT)
- device_type (TEXT)
- os_version (TEXT)
- status (TEXT): running/stopped
- last_used_at (INTEGER)
- created_at (INTEGER)
- updated_at (INTEGER)

### settings table
- key (TEXT PRIMARY KEY)
- value (TEXT)

## Code Style Guidelines

- Use Vue 3 Composition API with `<script setup>` syntax
- TypeScript strict mode enabled
- Follow Rust naming conventions in backend code
- Use i18n for all user-facing strings
- Support both light and dark themes

## Development Phases

1. **Phase 1**: Project setup, basic UI, settings, i18n, theming
2. **Phase 2**: Android emulator support
3. **Phase 3**: iOS emulator support (macOS)
4. **Phase 4**: HarmonyOS NEXT emulator support
5. **Phase 5**: System tray, auto-start, file watching, SQLite
6. **Phase 6**: Optimization and release packaging
