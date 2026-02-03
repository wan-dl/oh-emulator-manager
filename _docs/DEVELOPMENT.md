# 开发指南

## 项目状态

项目已完成基础框架搭建，包括：

### 已完成
- ✅ 项目结构创建
- ✅ 前端框架配置（Vue 3 + TypeScript + Vite）
- ✅ UI 组件库集成（Naive UI）
- ✅ 国际化支持（中文/英文）
- ✅ 路由配置
- ✅ 状态管理（Pinia）
- ✅ 主页面和设置页面
- ✅ 模拟器列表和卡片组件
- ✅ Tauri 后端框架
- ✅ Android 模拟器命令实现
- ✅ iOS 模拟器命令实现（macOS）
- ✅ HarmonyOS 模拟器命令实现
- ✅ 设置管理命令
- ✅ SQLite 数据库模块

### 待完成
- ⏳ Rust 依赖下载（首次构建需要时间）
- ⏳ 系统托盘功能
- ⏳ 文件监听功能
- ⏳ 日志查看器组件
- ⏳ 应用图标生成

## 开发步骤

### 1. 首次构建

由于 Rust 依赖较多，首次构建需要较长时间（10-30分钟）：

```bash
# 等待 Rust 依赖下载完成
cd src-tauri
cargo build

# 或直接运行开发模式（会自动构建）
cd ..
pnpm tauri:dev
```

### 2. 开发模式

```bash
# 启动开发服务器
pnpm tauri:dev
```

这会同时启动：
- Vite 开发服务器（前端热重载）
- Tauri 应用窗口

### 3. 类型检查

```bash
pnpm type-check
```

### 4. 构建生产版本

```bash
pnpm tauri:build
```

## 项目架构

### 前端（src/）
```
src/
├── components/          # Vue 组件
│   ├── EmulatorCard.vue    # 模拟器卡片
│   └── EmulatorList.vue    # 模拟器列表
├── views/              # 页面视图
│   ├── HomeView.vue        # 主页
│   └── SettingsView.vue    # 设置页
├── stores/             # Pinia 状态管理
│   ├── emulator.ts         # 模拟器状态
│   └── settings.ts         # 设置状态
├── i18n/               # 国际化
│   ├── zh-CN.json          # 中文
│   └── en-US.json          # 英文
├── router/             # 路由
│   └── index.ts
├── App.vue             # 根组件
└── main.ts             # 入口文件
```

### 后端（src-tauri/src/）
```
src-tauri/src/
├── commands/           # Tauri 命令
│   ├── android.rs          # Android 模拟器
│   ├── ios.rs              # iOS 模拟器
│   ├── harmony.rs          # HarmonyOS 模拟器
│   └── settings.rs         # 设置管理
├── db/                 # 数据库
│   └── mod.rs
└── main.rs             # 入口文件
```

## 功能实现说明

### Android 模拟器
- 使用 `emulator` 命令列出和启动 AVD
- 使用 `adb` 命令管理运行中的模拟器
- 支持截图、删除、清除数据

### iOS 模拟器（仅 macOS）
- 使用 `xcrun simctl` 命令管理
- 支持启动、关闭、删除、清除数据、截图
- 自动打开 Simulator.app

### HarmonyOS 模拟器
- 使用 `hdctool` 和 `hdc` 命令
- 支持列表、启动、关闭、截图
- 需要 DevEco Studio 5.x+

### 设置管理
- 语言切换（中文/英文）
- 主题切换（浅色/深色/跟随系统）
- SDK 路径配置
- 自动启动等选项

## 调试技巧

### 前端调试
- 在 Tauri 窗口中按 F12 打开开发者工具
- 使用 Vue DevTools 浏览器扩展

### 后端调试
- 查看 Rust 日志输出
- 使用 `println!` 或 `dbg!` 宏

### 常见问题

1. **Rust 编译失败**
   - 确保 Rust 版本 >= 1.70
   - 运行 `rustup update`

2. **前端类型错误**
   - 运行 `pnpm type-check`
   - 检查 TypeScript 配置

3. **模拟器命令失败**
   - 确保相关 SDK 已安装
   - 检查环境变量配置

## 下一步开发

1. 完成 Rust 依赖下载和首次编译
2. 测试各平台模拟器功能
3. 实现系统托盘功能
4. 添加文件监听自动刷新
5. 完善日志查看功能
6. 生成应用图标
7. 打包发布

## 贡献指南

欢迎提交 Issue 和 Pull Request！

开发时请遵循：
- Vue 3 Composition API 风格
- TypeScript 严格模式
- Rust 命名规范
- 代码格式化（Prettier + Rustfmt）
