# 快速启动指南

## 环境准备

### 必需工具
- Node.js 18+
- Rust 1.70+
- pnpm 8+

### 平台特定要求
- **macOS**: Xcode（用于 iOS 模拟器）
- **Windows/macOS**: Android SDK、DevEco Studio

## 快速开始

### 1. 安装依赖
```bash
pnpm install
```

### 2. 启动开发模式
```bash
pnpm tauri:dev
```

首次运行会下载 Rust 依赖，需要 10-30 分钟。

### 3. 构建生产版本
```bash
pnpm tauri:build
```

## 常用命令

```bash
pnpm dev              # 仅启动前端
pnpm build            # 构建前端
pnpm type-check       # 类型检查
pnpm tauri:dev        # 开发模式
pnpm tauri:build      # 生产构建
```

## 配置 SDK 路径

1. 启动应用
2. 点击右上角"设置"
3. 配置 Android SDK 和 DevEco Studio 路径
4. 保存设置

## 使用说明

1. 选择模拟器类型标签页
2. 点击"刷新"获取模拟器列表
3. 使用操作按钮管理模拟器

## 故障排除

### Rust 编译失败
```bash
rustup update
```

### 依赖安装失败
```bash
pnpm install --force
```

### 模拟器命令失败
- 检查 SDK 是否安装
- 验证环境变量配置
