# 修复截图打开功能

## 问题
点击控制台中的截图路径时，提示"无法打开图片"。

## 原因
使用 Tauri 的 `shell.open()` API 在某些情况下无法正确打开本地文件，特别是在 Windows 上。

## 解决方案

### 1. 创建后端命令
在 `src-tauri/src/commands/settings.rs` 中添加 `open_file` 命令：

```rust
#[tauri::command]
pub async fn open_file(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }
    
    Ok(())
}
```

### 2. 注册命令
在 `src-tauri/src/main.rs` 中注册命令：

```rust
settings::open_file,
```

### 3. 前端调用
在 `src/views/HomeView.vue` 中使用新命令：

```typescript
const openScreenshot = async (path: string) => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('open_file', { path })
  } catch (error) {
    // 失败时复制路径到剪贴板
    await navigator.clipboard.writeText(path)
    message.info('无法打开图片，路径已复制到剪贴板')
  }
}
```

### 4. 更新权限
在 `src-tauri/tauri.conf.json` 中添加必要的权限：

```json
"permissions": [
  "core:default",
  "shell:allow-open",
  "shell:allow-execute",
  "shell:allow-spawn",
  "dialog:allow-open",
  "dialog:allow-save"
]
```

## 工作原理

### Windows
使用 `explorer` 命令打开文件：
```
explorer C:\Users\...\screenshot.png
```

### macOS
使用 `open` 命令打开文件：
```
open /Users/.../screenshot.png
```

### Linux
使用 `xdg-open` 命令打开文件：
```
xdg-open /home/.../screenshot.png
```

## 优势

1. **跨平台兼容** - 使用各平台原生命令
2. **更可靠** - 直接调用系统命令，不依赖 shell 插件
3. **用户友好** - 失败时自动复制路径到剪贴板
4. **系统默认程序** - 使用用户设置的默认图片查看器

## 测试步骤

1. 重新构建应用：
```bash
pnpm tauri dev
```

2. 启动一个模拟器

3. 点击截图按钮

4. 在控制台中点击蓝色的文件路径

5. 验证图片是否在系统默认查看器中打开

## 备用方案

如果仍然无法打开：
- 路径会自动复制到剪贴板
- 用户可以手动粘贴路径到文件管理器
- 显示友好的提示信息

## 文件修改

- ✅ `src-tauri/src/commands/settings.rs` - 添加 `open_file` 命令
- ✅ `src-tauri/src/main.rs` - 注册命令
- ✅ `src/views/HomeView.vue` - 使用新命令
- ✅ `src-tauri/tauri.conf.json` - 更新权限

## 注意事项

- 确保文件路径存在且有效
- 文件必须是系统可以识别的格式
- 需要系统安装了默认的图片查看器
