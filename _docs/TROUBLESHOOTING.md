# 故障排除指南

## Windows 编译错误：linking with `link.exe` failed

### 问题描述
```
error: linking with `link.exe` failed: exit code: 1
note: link: extra operand '...'
Try 'link --help' for more information.
```

### 原因
Windows 系统中存在两个 `link.exe`：
1. Unix 工具的 `link` 命令（用于创建符号链接）
2. MSVC 的 `link.exe` 链接器（用于编译）

Rust 找到了错误的 `link.exe`。

### 解决方案

#### 方案 1：安装 Visual Studio Build Tools（推荐）

1. 下载 Visual Studio Build Tools：
   https://visualstudio.microsoft.com/downloads/

2. 运行安装程序，选择：
   - "使用 C++ 的桌面开发"
   - 确保勾选 "MSVC v143 - VS 2022 C++ x64/x86 生成工具"
   - 确保勾选 "Windows 10/11 SDK"

3. 安装完成后重启终端

4. 验证安装：
   ```bash
   where link
   ```
   应该显示 MSVC 的 link.exe 路径在前面

#### 方案 2：临时修复 PATH 环境变量

如果已安装 Visual Studio 但仍有问题：

1. 找到 MSVC link.exe 路径：
   ```
   C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\<version>\bin\Hostx64\x64\link.exe
   ```

2. 临时设置环境变量（在 PowerShell 中）：
   ```powershell
   $env:PATH = "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.XX.XXXXX\bin\Hostx64\x64;$env:PATH"
   ```

3. 重新运行：
   ```bash
   pnpm tauri:dev
   ```

#### 方案 3：使用 GNU 工具链

1. 切换到 GNU 工具链：
   ```bash
   rustup default stable-x86_64-pc-windows-gnu
   ```

2. 安装 MinGW-w64

3. 重新编译

### 验证修复

运行以下命令验证：
```bash
rustc --version
where link
```

确保 `link` 命令指向 MSVC 的链接器。

## 其他常见问题

### 问题：依赖下载缓慢

**解决方案**：配置国内镜像源

创建或编辑 `~/.cargo/config.toml`：
```toml
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

### 问题：前端类型错误

**解决方案**：
```bash
pnpm type-check
```

检查并修复 TypeScript 错误。

### 问题：模拟器命令失败

**解决方案**：
1. 确保已安装相应的 SDK
2. 配置环境变量：
   - `ANDROID_HOME` 或 `ANDROID_SDK_ROOT`
   - `DEVECO_SDK_HOME`
3. 验证命令可用：
   ```bash
   emulator -list-avds
   adb devices
   ```

### 问题：Rust 编译时间过长

**解决方案**：
1. 首次编译需要 10-30 分钟，这是正常的
2. 后续编译会快很多（增量编译）
3. 可以使用 `cargo build --release` 进行优化构建

## 获取帮助

如果问题仍未解决：
1. 查看完整错误日志
2. 搜索 Tauri 官方文档：https://tauri.app/
3. 访问 Tauri Discord 社区
4. 提交 GitHub Issue
