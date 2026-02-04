# 控制台标签页功能

## 功能说明

控制台现在支持两个标签页：
1. **程序输出** - 显示应用程序的操作日志（启动、停止、截图等）
2. **设备日志** - 显示 Android 设备的 logcat 日志

## UI 设计

### 布局结构
```
┌─────────────────────────────────┬────┐
│ 控制台              [清空] [✕] │    │
├─────────────────────────────────┤ 图 │
│                                 │ 标 │
│  日志内容区域                    │ 栏 │
│                                 │    │
│                                 │ ▣  │ ← 程序输出
│                                 │ ▢  │ ← 设备日志
└─────────────────────────────────┴────┘
```

### 图标栏
- 位置：控制台最右侧
- 宽度：48px
- 样式：垂直排列的图标按钮
- 激活状态：白色背景 + 绿色左边框

### 图标设计
1. **程序输出图标** - 文档/列表图标
2. **设备日志图标** - 手机/设备图标

## 功能实现

### 1. 程序输出标签
- ✅ 显示应用操作日志
- ✅ 支持不同类型：info、success、error、screenshot
- ✅ 截图日志可点击打开图片
- ✅ 自动滚动到最新日志

### 2. 设备日志标签
- ✅ 显示 Android logcat 日志
- ✅ 自动检测运行中的模拟器
- ✅ 实时更新（每秒刷新）
- ✅ 限制日志数量（最多 1000 条）
- ✅ 自动滚动到最新日志
- ✅ 组件卸载时自动停止日志收集

### 3. 清空功能
- 根据当前标签清空对应的日志
- 程序输出标签 → 清空程序日志
- 设备日志标签 → 清空设备日志

## 后端实现

### Android Logcat 命令

#### 1. start_logcat
```rust
#[tauri::command]
pub async fn start_logcat(device_id: String) -> Result<(), String>
```
- 启动 adb logcat 监听
- 使用后台线程收集日志
- 日志存储在全局缓冲区

#### 2. get_logcat_logs
```rust
#[tauri::command]
pub async fn get_logcat_logs(device_id: String) -> Result<Vec<String>, String>
```
- 获取缓冲区中的日志
- 返回后清空缓冲区
- 前端定期调用（每秒）

#### 3. stop_logcat
```rust
#[tauri::command]
pub async fn stop_logcat() -> Result<(), String>
```
- 停止日志收集
- 清空缓冲区
- 组件卸载时调用

### 全局状态
```rust
static LOGCAT_BUFFER: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LOGCAT_RUNNING: Mutex<bool> = Mutex::new(false);
```

## 使用方法

### 查看程序输出
1. 点击控制台图标展开控制台
2. 默认显示程序输出标签
3. 查看应用操作日志

### 查看设备日志
1. 启动一个 Android 模拟器
2. 点击控制台右侧的"设备日志"图标
3. 自动开始收集 logcat 日志
4. 实时显示设备日志

### 清空日志
- 点击"清空"按钮清空当前标签的日志

## 样式说明

### 标签栏样式
```css
.console-tabs {
  width: 48px;
  border-left: 1px solid #e0e0e0;
  background: #f0f0f0;
}

.console-tab {
  height: 44px;
  color: #666;
  border-left: 3px solid transparent;
}

.console-tab.active {
  background: white;
  color: #18a058;
  border-left-color: #18a058;
}
```

### 日志样式
- 程序日志：彩色（根据类型）
- 设备日志：灰色单色
- 字体：Monaco, Menlo, Ubuntu Mono（等宽字体）
- 字号：12px（消息）、11px（时间）

## 性能优化

1. **日志限制**
   - 程序日志：无限制（用户操作较少）
   - 设备日志：最多 1000 条（自动删除旧日志）

2. **更新频率**
   - 设备日志：每秒更新一次
   - 避免过于频繁的更新影响性能

3. **自动清理**
   - 组件卸载时停止 logcat
   - 切换标签时不停止收集（保持后台运行）

## 文件修改

- ✅ `src/views/HomeView.vue` - 添加标签页 UI 和逻辑
- ✅ `src-tauri/src/commands/android.rs` - 添加 logcat 命令
- ✅ `src-tauri/src/main.rs` - 注册新命令

## 测试步骤

1. 启动应用：`pnpm tauri dev`
2. 启动一个 Android 模拟器
3. 点击控制台图标展开
4. 默认显示程序输出
5. 点击右侧"设备日志"图标
6. 验证是否显示 logcat 日志
7. 测试清空功能
8. 测试标签切换

## 未来扩展

- [ ] iOS 设备日志支持
- [ ] HarmonyOS 设备日志支持
- [ ] 日志过滤功能
- [ ] 日志搜索功能
- [ ] 日志导出功能
- [ ] 日志级别筛选（Error、Warning、Info 等）
