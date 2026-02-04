# 修复 macOS 侧边栏折叠边距

## 问题
`.device-header-left.sidebar-collapsed` 的 `margin-left: 70px` 在所有平台上都生效，但这个边距只应该在 macOS 上生效。

## 原因
macOS 窗口左上角有红绿灯按钮（关闭、最小化、最大化），当侧边栏折叠时，需要留出空间避免内容被按钮遮挡。但在 Windows 和 Linux 上不需要这个边距。

## 解决方案

### 1. 使用动态样式绑定
将固定的 CSS 规则改为动态的 style 绑定：

```vue
<div 
  class="device-header-left" 
  :class="{ 'sidebar-collapsed': sidebarCollapsed }"
  :style="{ marginLeft: (sidebarCollapsed && isMacOS) ? '70px' : '0' }"
>
```

### 2. 移除固定 CSS
删除原来的 CSS 规则：

```css
/* 删除这个规则 */
.device-header-left.sidebar-collapsed {
  margin-left: 70px;
}
```

### 3. 保留过渡效果
保持平滑的过渡动画：

```css
.device-header-left {
  display: flex;
  align-items: center;
  gap: 8px;
  transition: margin-left 0.3s ease;
}
```

## 逻辑说明

### 条件判断
```typescript
marginLeft: (sidebarCollapsed && isMacOS) ? '70px' : '0'
```

- `sidebarCollapsed = true` 且 `isMacOS = true` → `margin-left: 70px`
- 其他情况 → `margin-left: 0`

### 平台检测
```typescript
const isMacOS = ref(false)

onMounted(async () => {
  const userAgent = navigator.userAgent.toLowerCase()
  isMacOS.value = userAgent.includes('mac')
})
```

## 效果对比

### macOS
- 侧边栏展开：`margin-left: 0`
- 侧边栏折叠：`margin-left: 70px` ✅ 避开红绿灯按钮

### Windows / Linux
- 侧边栏展开：`margin-left: 0`
- 侧边栏折叠：`margin-left: 0` ✅ 不需要额外边距

## 为什么需要这个边距？

### macOS 窗口布局
```
┌─ ● ● ● ──────────────────┐
│  ↑                        │
│  红绿灯按钮                │
│                           │
│  [标题] [展开按钮]         │
│  ↑                        │
│  需要避开按钮区域           │
└───────────────────────────┘
```

### Windows 窗口布局
```
┌───────────────────── _ □ ✕
│                           │
│  [标题] [展开按钮]         │
│  ↑                        │
│  不需要额外边距             │
└───────────────────────────┘
```

## 文件修改
- ✅ `src/views/HomeView.vue` - 添加动态样式绑定，移除固定 CSS

## 测试步骤

### macOS 测试
1. 启动应用
2. 折叠侧边栏
3. 验证标题左移 70px，避开红绿灯按钮
4. 展开侧边栏
5. 验证标题恢复到左边缘

### Windows 测试
1. 启动应用
2. 折叠侧边栏
3. 验证标题保持在左边缘（无额外边距）
4. 展开侧边栏
5. 验证标题保持在左边缘

## 技术细节

### 动态样式优先级
内联样式（`:style`）的优先级高于 CSS 类，确保动态值能够覆盖默认样式。

### 过渡动画
保留 `transition: margin-left 0.3s ease` 确保边距变化时有平滑的动画效果。

### 响应式
使用 `isMacOS` ref 确保平台检测是响应式的，虽然在实际使用中这个值不会改变。
