# Windows 编译问题快速修复

## 当前问题
Rust 编译时找到了错误的 `link.exe`（Unix 工具而非 MSVC 链接器）

## 立即修复步骤

### 步骤 1：检查是否已安装 Visual Studio Build Tools

打开 PowerShell 运行：
```powershell
where link
```

如果输出类似：
```
C:\Windows\System32\link.exe
```
说明找到的是 Unix 工具，需要安装 MSVC。

### 步骤 2：安装 Visual Studio Build Tools

1. 访问：https://visualstudio.microsoft.com/zh-hans/downloads/

2. 下载 "Visual Studio 2022 生成工具"

3. 运行安装程序，选择：
   - ✅ 使用 C++ 的桌面开发
   - ✅ MSVC v143 - VS 2022 C++ x64/x86 生成工具
   - ✅ Windows 11 SDK (或 Windows 10 SDK)

4. 安装大小约 6-8 GB，需要 15-30 分钟

### 步骤 3：重启终端并验证

关闭所有终端窗口，重新打开 PowerShell：

```powershell
where link
```

现在应该显示：
```
C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\...\link.exe
C:\Windows\System32\link.exe
```

MSVC 的 link.exe 应该在第一位。

### 步骤 4：重新编译

```bash
cd D:\_project\oh-emulator-manager
pnpm tauri:dev
```

## 替代方案：使用 GNU 工具链

如果不想安装 Visual Studio：

```bash
# 切换到 GNU 工具链
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu

# 安装 MinGW-w64
# 下载：https://www.mingw-w64.org/downloads/

# 重新编译
pnpm tauri:dev
```

## 预期结果

安装完成后，编译应该能够正常进行：
```
Compiling proc-macro2 v1.0.106
Compiling quote v1.0.44
Compiling serde v1.0.228
...
Finished dev [unoptimized + debuginfo] target(s) in 15m 30s
```

首次编译需要 10-30 分钟，请耐心等待。

## 验证安装

```bash
# 检查 Rust 工具链
rustc --version
rustup show

# 检查链接器
where link

# 检查 MSVC
cl
```

## 需要帮助？

参考 TROUBLESHOOTING.md 获取更多解决方案。
