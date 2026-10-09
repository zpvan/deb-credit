# Material 3「童趣版」界面改造设计

日期：2026-10-09

## 目标

将应用界面整体改为 Google Material 3（Material You）风格，保留童趣性格：暖橙主题、Baloo 2 圆体、emoji 图标唱主角。只改视觉，不动布局与信息架构。

## 决策记录

| 决策点 | 结论 |
|---|---|
| Material 代际 | Material 3（Material You），现行标准 |
| 风格性格 | 童趣 M3：M3 骨架 + 暖色主题 + Baloo 2 圆体 + emoji 唱主角 |
| 主题色 | 暖橙（seed ≈ #f97316），派生整套 tonal 色板 |
| 深色模式 | 支持，可手动切换；初始值读 localStorage，无记录跟随系统 |
| 实现方案 | A：M3 design tokens（CSS 变量）+ @apply 语义组件类，Rust 组件换类名 |

## 设计内容

### 1. 设计 Tokens（CSS 变量）

- **色彩**：暖橙 seed 按 Material You 规范派生语义角色变量，每个角色有亮色/暗色两套取值：
  `primary / on-primary / primary-container / on-primary-container / secondary / secondary-container / surface / surface-container-lowest / surface-container-low / surface-container-high / surface-container-highest / on-surface / on-surface-variant / outline / outline-variant / error / on-error`
- **圆角**：M3 形状体系——卡片 16–20px，按钮/Chip 全圆角（pill），对话框 28px
- **阴影**：M3 elevation 1–3 级，低阴影 + tonal 色块层次为主
- **字体**：Baloo 2 作展示字体（标题、积分数字），正文保持系统中文字体栈（PingFang SC 等）

### 2. CSS 管线修复（前置工作）

`public/main.css` 当前是提交进仓库的编译产物，无 `@tailwind` 指令，新 class 不生效。改为真正的 Tailwind 源文件：

```
@import 字体
@tailwind base; @tailwind components; @tailwind utilities;
@layer base { M3 tokens（:root 与 .dark） }
@layer components { 语义组件类 }
```

`tailwind.config.js` 的 shadcn 变量映射替换为 M3 角色映射（colors 指向新 CSS vars，darkMode 用 `class` 策略）。构建机制不变（trunk 调 tailwind 编译，`TRUNK_TOOLS_TAILWINDCSS` 照旧）。

### 3. 语义组件类（@layer components）

`m3-card`、`m3-btn-filled`、`m3-btn-tonal`、`m3-btn-outlined`、`m3-btn-text`、`m3-fab`、`m3-chip`、`m3-dialog`（含 scrim）、`m3-check`（圆形打卡按钮）、`m3-tab`（孩子切换，pill 指示器）、`m3-icon-btn`、`m3-switch`（暗色切换）、`m3-divider`。

### 4. 组件改造范围

只换类名与视觉，不改布局结构、信息架构和交互逻辑：

- `src/app.rs`：顶栏 + 孩子 tabs + 新增暗色切换 icon button
- `src/sections/`：today_tasks、calendar_board、charts、wishes、history
- `src/components/`：dialog、item_dialog、kid_dialog、tabs
- `src/charts_svg.rs`：图表颜色改用 CSS 变量（适配暗色）
- `src/icons.rs`：不动（补充 sun/moon 图标如缺失）

### 5. 暗色模式

- `html` 元素挂 `.dark` 类切换；`tailwind.config.js` 设 `darkMode: 'class'`
- 初始化读 localStorage `theme`，无值则跟随 `prefers-color-scheme`
- 顶栏 sun/moon icon button 手动切换并写回 localStorage

### 6. 验证

- `cargo test`：数据层不受影响，应保持全绿
- `TRUNK_TOOLS_TAILWINDCSS=... trunk build --release` 通过（CI 部署管线不受影响）
- `trunk serve` 人工检查 5 个 tab 在亮色/暗色下的表现

## 约束

- 不改任何数据层（models / store / persist / credits / date），备份 JSON 格式不变
- 不引入新 Rust 依赖；纯 CSS + 类名调整（暗色切换逻辑用现有 web-sys 能力）
- 布局、文案、交互流程不变

## 风险

- `public/main.css` 从产物改源文件后，若 tailwind 编译失败会导致无样式——每步构建验证
- 暖橙在暗色下需注意对比度（on-primary 用深棕而非纯黑/白，按 M3 规范取值）
