# 最快速的 MinGW 安装方案

## 使用 Scoop 一键安装（最简单）

### 1. 安装 Scoop（如果还没有）

打开 PowerShell（管理员权限），运行：

```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex
```

### 2. 安装 MinGW

```powershell
scoop install mingw
```

就这么简单！Scoop 会自动：
- 下载 MinGW-w64
- 配置环境变量
- 设置好所有路径

### 3. 切换 Rust 工具链

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### 4. 开始开发

```bash
cd D:\_project\oh-emulator-manager
pnpm tauri:dev
```

## 优势

- ✅ 最简单：只需 2 条命令
- ✅ 最快：5 分钟搞定
- ✅ 最小：约 300-500 MB
- ✅ 自动配置：无需手动设置 PATH

## 验证

```bash
gcc --version
rustup show
```

## 其他有用的 Scoop 命令

```bash
# 查看已安装
scoop list

# 更新 MinGW
scoop update mingw

# 卸载
scoop uninstall mingw
```
