# 立即安装 GNU 工具链 - 完整步骤

## 当前状态
✅ Rust 已安装（MSVC 版本）
⏳ 正在安装 GNU 工具链...

## 完整步骤（请按顺序执行）

### 步骤 1：安装 GNU 工具链（正在进行）

当前命令正在运行，请等待完成：
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
```

这会下载约 200-300 MB 的文件。

**如果下载太慢，可以按 Ctrl+C 取消，然后：**

#### 方案 A：使用国内镜像加速

在 PowerShell 中设置环境变量：
```powershell
$env:RUSTUP_DIST_SERVER="https://mirrors.tuna.tsinghua.edu.cn/rustup"
$env:RUSTUP_UPDATE_ROOT="https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup"
rustup toolchain install stable-x86_64-pc-windows-gnu
```

#### 方案 B：使用中科大镜像
```powershell
$env:RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static"
$env:RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup"
rustup toolchain install stable-x86_64-pc-windows-gnu
```

### 步骤 2：安装 MinGW-w64（必需）

GNU 工具链需要 GCC 编译器。选择以下任一方案：

#### 方案 A：使用 Scoop（推荐，最简单）

```powershell
# 安装 Scoop（如果还没有）
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex

# 安装 MinGW
scoop install mingw

# 验证
gcc --version
```

#### 方案 B：手动下载安装

1. 下载 MinGW-w64：
   https://github.com/niXman/mingw-builds-binaries/releases

2. 选择文件：
   `x86_64-13.2.0-release-posix-seh-ucrt-rt_v11-rev0.7z`

3. 解压到：`C:\mingw64`

4. 添加到 PATH：
   - 右键"此电脑" → 属性 → 高级系统设置
   - 环境变量 → 系统变量 → Path → 编辑
   - 新建：`C:\mingw64\bin`
   - 确定保存

5. 重启 PowerShell，验证：
   ```bash
   gcc --version
   ```

### 步骤 3：切换 Rust 默认工具链

等步骤 1 完成后，运行：

```bash
# 设置 GNU 为默认
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

### 步骤 4：清理并重新编译

```bash
cd D:\_project\oh-emulator-manager

# 清理之前的编译缓存
cd src-tauri
cargo clean
cd ..

# 开始开发
pnpm tauri:dev
```

## 预期时间

- 步骤 1：5-15 分钟（取决于网速）
- 步骤 2：2-5 分钟
- 步骤 3：1 分钟
- 步骤 4：首次编译 10-20 分钟

## 验证成功

当你看到以下输出，说明成功：

```
Compiling oh-emulator-manager v0.1.0
Finished dev [unoptimized + debuginfo] target(s) in 15m 30s
```

应用窗口会自动打开！

## 遇到问题？

### GCC 找不到
```bash
# 检查 PATH
echo $env:PATH

# 应该包含 MinGW 路径
```

### 编译错误
```bash
# 清理重试
cd src-tauri
cargo clean
cargo build
```

### 切换回 MSVC
```bash
rustup default stable-x86_64-pc-windows-msvc
```

## 总结

- ✅ 无需安装 6-8 GB 的 Visual Studio
- ✅ 只需 500 MB 左右
- ✅ 完全支持 Tauri 开发
- ✅ 编译速度相当

继续等待步骤 1 完成，或使用镜像加速！
