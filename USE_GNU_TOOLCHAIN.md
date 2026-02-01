# 使用 GNU 工具链（轻量方案）

## 为什么选择 GNU 工具链？

- ✅ 体积小：约 500 MB（vs MSVC 的 6-8 GB）
- ✅ 安装快：5-10 分钟
- ✅ 功能完整：支持所有 Tauri 功能
- ⚠️ 注意：某些 Windows 特定功能可能需要 MSVC

## 安装步骤

### 1. 安装 MSYS2（包含 MinGW-w64）

下载并安装 MSYS2：
https://www.msys2.org/

或直接下载：
https://github.com/msys2/msys2-installer/releases/download/2024-01-13/msys2-x86_64-20240113.exe

安装到默认位置：`C:\msys64`

### 2. 安装 MinGW-w64 工具链

打开 MSYS2 终端（开始菜单搜索 "MSYS2 MSYS"），运行：

```bash
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain
```

### 3. 添加到 PATH

将以下路径添加到系统环境变量 PATH：
```
C:\msys64\mingw64\bin
```

**如何添加：**
1. 右键"此电脑" → 属性
2. 高级系统设置 → 环境变量
3. 系统变量中找到 Path，点击编辑
4. 新建，添加 `C:\msys64\mingw64\bin`
5. 确定保存

### 4. 切换 Rust 工具链

打开 PowerShell：

```bash
# 安装 GNU 工具链
rustup toolchain install stable-x86_64-pc-windows-gnu

# 设置为默认
rustup default stable-x86_64-pc-windows-gnu

# 验证
rustup show
```

应该显示：
```
active toolchain
----------------
stable-x86_64-pc-windows-gnu (default)
```

### 5. 重新编译

```bash
cd D:\_project\oh-emulator-manager
pnpm tauri:dev
```

## 验证安装

```bash
# 检查 GCC
gcc --version

# 检查 Rust 工具链
rustc --version
rustup show
```

## 如果遇到问题

### 问题：找不到 gcc

**解决：** 确保 `C:\msys64\mingw64\bin` 在 PATH 中，重启终端

### 问题：编译错误

**解决：** 清理并重新编译
```bash
cd src-tauri
cargo clean
cd ..
pnpm tauri:dev
```

## 切换回 MSVC（如果需要）

```bash
rustup default stable-x86_64-pc-windows-msvc
```

## 总结

- 安装大小：~500 MB
- 安装时间：5-10 分钟
- 完全支持 Tauri 开发
- 推荐用于个人开发和学习
