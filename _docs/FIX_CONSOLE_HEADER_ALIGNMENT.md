# 修复控制台标题对齐

## 问题
控制台标题显示在中间位置，而不是在开始位置（左对齐）。

## 原因
使用了 `justify-content: space-between`，导致折叠按钮、标题、操作按钮三个元素分散对齐，标题被推到了中间。

## 解决方案

### 1. 重构 HTML 结构
将折叠按钮和标题包裹在一个容器中：

```vue
<div class="console-header">
  <div class="console-header-left">
    <n-button @click="consoleCollapsed = true">
      <img src="@/assets/silder.svg" />
    </n-button>
    <span class="console-title">控制台</span>
  </div>
  <div class="console-header-actions">
    <n-button @click="clearConsole">清空</n-button>
  </div>
</div>
```

### 2. 更新样式

```css
.console-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  height: 61px;
}

.console-header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.console-title {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}
```

## 效果

### 修改前
```
[折叠按钮]        控制台        [清空]
```

### 修改后
```
[折叠按钮] 控制台              [清空]
```

## 布局说明

- **左侧**: 折叠按钮 + 标题（紧密排列，间距 8px）
- **右侧**: 清空按钮
- **对齐**: 使用 `justify-content: space-between` 让左右两组分别靠边

## 文件修改
- ✅ `src/views/HomeView.vue` - 重构控制台头部结构和样式

## 测试
```bash
pnpm tauri dev
```

展开控制台，验证标题和折叠按钮都在左侧对齐。
