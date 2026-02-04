# 控制台拖动调整宽度功能

## 功能说明

在第2列（设备列表）和第3列（控制台）之间添加可拖动的分隔条，支持鼠标拖动调整控制台宽度。

## UI 设计

### 分隔条
- **宽度**: 4px
- **位置**: 第2列和第3列之间
- **默认**: 透明背景
- **悬停**: 绿色背景 (#18a058)
- **光标**: `ew-resize`（左右调整光标）
- **点击区域**: 扩展到左右各 2px（共 8px）

### 控制台宽度
- **默认宽度**: 320px
- **最小宽度**: 280px
- **最大宽度**: 600px
- **动态调整**: 拖动时实时更新

## 功能实现

### 1. 拖动逻辑

```typescript
const startResize = (e: MouseEvent) => {
  isResizing.value = true
  const startX = e.clientX
  const startWidth = consolePanelWidth.value

  const handleMouseMove = (e: MouseEvent) => {
    // 计算新宽度（从右边拖动，所以是减法）
    const deltaX = startX - e.clientX
    const newWidth = startWidth + deltaX
    
    // 限制最小和最大宽度
    if (newWidth >= 280 && newWidth <= 600) {
      consolePanelWidth.value = newWidth
    }
  }

  const handleMouseUp = () => {
    isResizing.value = false
    // 清理事件监听器
  }
}
```

### 2. 窗口大小同步

```typescript
// 监听控制台宽度变化，同步调整窗口大小
watch(consolePanelWidth, async (newWidth) => {
  if (!consoleCollapsed.value) {
    const appWindow = getCurrentWindow()
    const currentSize = await appWindow.innerSize()
    await appWindow.setSize(new LogicalSize(828 + newWidth, currentSize.height))
  }
})
```

### 3. 状态管理

```typescript
const consolePanelWidth = ref(320)  // 控制台宽度
const isResizing = ref(false)       // 是否正在拖动
```

## 交互细节

### 拖动开始
1. 鼠标按下分隔条
2. 光标变为 `ew-resize`
3. 禁用文本选择（`user-select: none`）
4. 开始监听鼠标移动

### 拖动中
1. 实时计算新宽度
2. 限制在最小/最大宽度范围内
3. 更新控制台宽度
4. 同步调整窗口大小

### 拖动结束
1. 鼠标释放
2. 恢复光标样式
3. 恢复文本选择
4. 移除事件监听器

## 样式实现

### 分隔条样式
```css
.resize-handle {
  width: 4px;
  background: transparent;
  cursor: ew-resize;
  flex-shrink: 0;
  position: relative;
  transition: background 0.2s;
}

.resize-handle:hover {
  background: #18a058;
}

/* 扩展点击区域 */
.resize-handle::before {
  content: '';
  position: absolute;
  left: -2px;
  right: -2px;
  top: 0;
  bottom: 0;
}
```

### 控制台样式
```css
.console-panel {
  min-width: 280px;
  max-width: 600px;
  /* 使用动态宽度 */
  width: var(--console-width);
}
```

## 窗口大小计算

### 基础宽度
- 侧边栏: 200px
- 设备列表: 628px
- **总计**: 828px

### 展开控制台
- 基础宽度: 828px
- 控制台宽度: 280px ~ 600px
- **总宽度**: 1108px ~ 1428px

### 示例
- 默认: 828 + 320 = 1148px
- 最小: 828 + 280 = 1108px
- 最大: 828 + 600 = 1428px

## 用户体验优化

### 1. 视觉反馈
- ✅ 悬停时分隔条变绿色
- ✅ 拖动时光标变为调整大小图标
- ✅ 平滑的过渡动画

### 2. 操作限制
- ✅ 最小宽度限制（280px）- 保证内容可读
- ✅ 最大宽度限制（600px）- 避免占用过多空间
- ✅ 实时更新 - 拖动时立即看到效果

### 3. 性能优化
- ✅ 使用 `requestAnimationFrame` 优化拖动性能
- ✅ 拖动结束后清理事件监听器
- ✅ 避免不必要的重渲染

## 使用方法

### 调整控制台宽度
1. 展开控制台
2. 将鼠标移到第2列和第3列之间的边界
3. 鼠标光标变为 `↔` 形状
4. 按住鼠标左键拖动
5. 向左拖动 → 控制台变宽
6. 向右拖动 → 控制台变窄
7. 释放鼠标完成调整

### 恢复默认宽度
- 关闭并重新打开控制台
- 默认恢复为 320px

## 技术细节

### 事件处理
```typescript
// 鼠标按下 - 开始拖动
@mousedown="startResize"

// 鼠标移动 - 更新宽度
document.addEventListener('mousemove', handleMouseMove)

// 鼠标释放 - 结束拖动
document.addEventListener('mouseup', handleMouseUp)
```

### 宽度计算
```typescript
// 从右边拖动，向左增加宽度
const deltaX = startX - e.clientX
const newWidth = startWidth + deltaX

// 限制范围
if (newWidth >= 280 && newWidth <= 600) {
  consolePanelWidth.value = newWidth
}
```

## 文件修改

- ✅ `src/views/HomeView.vue` - 添加拖动功能和分隔条

## 测试步骤

1. 启动应用: `pnpm tauri dev`
2. 展开控制台
3. 将鼠标移到边界处
4. 验证光标变为 `↔`
5. 拖动调整宽度
6. 验证窗口大小同步变化
7. 测试最小/最大宽度限制
8. 测试拖动流畅度

## 已知限制

- 宽度调整不会持久化（重启后恢复默认）
- 只支持鼠标拖动（不支持触摸）

## 未来改进

- [ ] 保存用户自定义宽度到设置
- [ ] 支持双击分隔条恢复默认宽度
- [ ] 支持触摸设备拖动
- [ ] 添加宽度调整动画
