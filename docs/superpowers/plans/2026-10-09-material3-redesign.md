# Material 3「童趣版」界面改造实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将应用界面整体改为 Material 3（暖橙童趣版），含暗色模式手动切换。

**Architecture:** 重建 CSS 管线（`public/main.css` 从编译产物改为 Tailwind 源文件），定义 M3 暖橙 design tokens（CSS 变量，亮/暗两套）+ `@layer components` 语义类（m3-card、m3-btn-* 等），然后逐文件把硬编码颜色类替换为 M3 token 类。布局与交互逻辑不变。

**Tech Stack:** Tailwind CSS v3、Leptos 0.8 (CSR)、web-sys

**Spec:** `docs/superpowers/specs/2026-10-09-material3-redesign-design.md`

**构建/验证命令（全计划通用）：**

```bash
export TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss
cargo test                # 数据层 + 图表测试
trunk build               # 快速构建验证（dev）
```

**颜色映射约定（旧 → 新 token，后续各 task 使用）：**

| 旧硬编码 | 新 token |
|---|---|
| `#1d5d3f`（主绿/赚得） | `earn` |
| `#f8622f`（橙/花掉/强调） | `primary` |
| `#ecc22e`（金） | 选中态高亮，多用 `on-*` 反色替代 |
| `#9274b1`（紫） | `on-surface-variant` 或 `primary` |
| `#872020`（红） | `error` |
| `#4c87bf`（蓝） | `primary` |
| `#feffc9`（浅黄底） | `secondary-container` |
| `#f3ecfa`（浅紫底） | `secondary-container` |
| `text-muted-foreground` | `text-on-surface-variant` |
| `bg-muted` | `bg-surface-container-high` |
| `bg-card`/`bg-background` | `m3-card` 或 `surface` 系 |
| `border-border` | `border-outline-variant` |

---

### Task 1: CSS 管线重建 + M3 tokens + 语义组件类

**Files:**
- Modify: `public/main.css`（整体重写为 Tailwind 源文件）
- Modify: `tailwind.config.js`（整体重写）

- [ ] **Step 1: 重写 `tailwind.config.js`**

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./index.html', './src/**/*.rs'],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: 'rgb(var(--primary) / <alpha-value>)',
          container: 'rgb(var(--primary-container) / <alpha-value>)',
        },
        'on-primary': 'rgb(var(--on-primary) / <alpha-value>)',
        'on-primary-container': 'rgb(var(--on-primary-container) / <alpha-value>)',
        'secondary-container': 'rgb(var(--secondary-container) / <alpha-value>)',
        'on-secondary-container': 'rgb(var(--on-secondary-container) / <alpha-value>)',
        earn: {
          DEFAULT: 'rgb(var(--earn) / <alpha-value>)',
          container: 'rgb(var(--earn-container) / <alpha-value>)',
        },
        'on-earn': 'rgb(var(--on-earn) / <alpha-value>)',
        'on-earn-container': 'rgb(var(--on-earn-container) / <alpha-value>)',
        surface: 'rgb(var(--surface) / <alpha-value>)',
        'surface-container-lowest': 'rgb(var(--surface-container-lowest) / <alpha-value>)',
        'surface-container-low': 'rgb(var(--surface-container-low) / <alpha-value>)',
        'surface-container-high': 'rgb(var(--surface-container-high) / <alpha-value>)',
        'surface-container-highest': 'rgb(var(--surface-container-highest) / <alpha-value>)',
        'on-surface': 'rgb(var(--on-surface) / <alpha-value>)',
        'on-surface-variant': 'rgb(var(--on-surface-variant) / <alpha-value>)',
        outline: 'rgb(var(--outline) / <alpha-value>)',
        'outline-variant': 'rgb(var(--outline-variant) / <alpha-value>)',
        error: {
          DEFAULT: 'rgb(var(--error) / <alpha-value>)',
          container: 'rgb(var(--error-container) / <alpha-value>)',
        },
        'on-error': 'rgb(var(--on-error) / <alpha-value>)',
        'on-error-container': 'rgb(var(--on-error-container) / <alpha-value>)',
      },
      fontFamily: {
        display: ["'Baloo 2'", "'PingFang SC'", "'Microsoft YaHei'", 'sans-serif'],
      },
      boxShadow: {
        'elevation-1': '0 1px 2px rgb(60 32 0 / 0.10), 0 1px 3px rgb(60 32 0 / 0.06)',
        'elevation-2': '0 2px 6px rgb(60 32 0 / 0.12), 0 1px 3px rgb(60 32 0 / 0.08)',
        'elevation-3': '0 4px 12px rgb(60 32 0 / 0.16)',
      },
    },
  },
}
```

- [ ] **Step 2: 重写 `public/main.css`**

```css
@import url('https://fonts.googleapis.com/css2?family=Baloo+2:wght@500;600;700;800&display=swap');

@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    /* Material 3 暖橙主题（亮色）。值为 RGB 三元组，配合 tailwind <alpha-value> 使用 */
    --primary: 152 72 0;
    --on-primary: 255 255 255;
    --primary-container: 255 220 194;
    --on-primary-container: 49 19 0;
    --secondary-container: 255 229 212;
    --on-secondary-container: 43 22 11;
    --earn: 47 107 60;
    --on-earn: 255 255 255;
    --earn-container: 215 236 212;
    --on-earn-container: 11 46 18;
    --surface: 255 248 242;
    --surface-container-lowest: 255 255 255;
    --surface-container-low: 255 239 226;
    --surface-container-high: 249 227 208;
    --surface-container-highest: 243 217 196;
    --on-surface: 35 26 17;
    --on-surface-variant: 86 67 51;
    --outline: 138 113 94;
    --outline-variant: 221 192 170;
    --error: 186 26 26;
    --on-error: 255 255 255;
    --error-container: 255 218 214;
    --on-error-container: 65 0 2;
  }

  .dark {
    --primary: 255 184 121;
    --on-primary: 74 40 0;
    --primary-container: 109 58 0;
    --on-primary-container: 255 220 194;
    --secondary-container: 92 56 32;
    --on-secondary-container: 255 229 212;
    --earn: 147 207 155;
    --on-earn: 11 46 18;
    --earn-container: 29 74 38;
    --on-earn-container: 215 236 212;
    --surface: 26 18 11;
    --surface-container-lowest: 20 13 7;
    --surface-container-low: 34 26 18;
    --surface-container-high: 45 36 27;
    --surface-container-highest: 56 46 35;
    --on-surface: 241 223 208;
    --on-surface-variant: 215 194 177;
    --outline: 159 141 126;
    --outline-variant: 85 67 47;
    --error: 255 180 171;
    --on-error: 105 0 5;
    --error-container: 147 0 10;
    --on-error-container: 255 218 214;
  }

  body {
    @apply bg-surface text-on-surface antialiased;
    font-family: 'Baloo 2', 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', 'Source Han Sans SC', sans-serif;
    font-feature-settings: 'tnum';
  }
}

@layer components {
  .m3-card {
    @apply rounded-2xl bg-surface-container-lowest text-on-surface shadow-elevation-1;
  }
  .m3-btn-filled {
    @apply inline-flex h-10 items-center justify-center gap-2 whitespace-nowrap rounded-full bg-primary px-5 font-display text-sm font-bold text-on-primary shadow-elevation-1 transition-all hover:shadow-elevation-2 disabled:pointer-events-none disabled:opacity-50;
  }
  .m3-btn-tonal {
    @apply inline-flex h-10 items-center justify-center gap-2 whitespace-nowrap rounded-full bg-secondary-container px-5 font-display text-sm font-bold text-on-secondary-container transition-all hover:shadow-elevation-1 disabled:pointer-events-none disabled:opacity-50;
  }
  .m3-btn-outlined {
    @apply inline-flex h-10 items-center justify-center gap-2 whitespace-nowrap rounded-full border border-outline bg-transparent px-4 font-display text-sm font-bold text-primary transition-all hover:bg-primary/10 disabled:pointer-events-none disabled:opacity-50;
  }
  .m3-btn-danger {
    @apply inline-flex h-10 items-center justify-center gap-2 whitespace-nowrap rounded-full bg-error px-5 font-display text-sm font-bold text-on-error shadow-elevation-1 transition-all hover:shadow-elevation-2 disabled:pointer-events-none disabled:opacity-50;
  }
  .m3-chip {
    @apply inline-flex items-center gap-1 rounded-full bg-secondary-container px-2.5 py-1 font-display text-xs font-bold text-on-secondary-container;
  }
  .m3-icon-btn {
    @apply inline-flex h-10 w-10 items-center justify-center rounded-full text-on-surface-variant transition-colors hover:bg-on-surface/10;
  }
  .m3-input {
    @apply h-11 w-full rounded-xl border border-outline bg-surface-container-lowest px-3 text-on-surface outline-none transition-shadow placeholder:text-on-surface-variant/70 focus:border-primary focus:ring-2 focus:ring-primary/30;
  }
}

@layer utilities {
  .animate-pop-in {
    animation: pop-in 0.35s cubic-bezier(0.34, 1.56, 0.64, 1) both;
  }
  .animate-float-coin {
    animation: float-coin 3.2s ease-in-out infinite;
  }
  .animate-wiggle {
    animation: wiggle 0.5s ease-in-out;
  }
}

@keyframes pop-in {
  0% { transform: scale(0.6); opacity: 0; }
  60% { transform: scale(1.08); }
  100% { transform: scale(1); opacity: 1; }
}

@keyframes float-coin {
  0%, 100% { transform: translateY(0) rotate(-3deg); }
  50% { transform: translateY(-6px) rotate(3deg); }
}

@keyframes wiggle {
  0%, 100% { transform: rotate(0deg); }
  25% { transform: rotate(-4deg); }
  75% { transform: rotate(4deg); }
}
```

- [ ] **Step 3: 构建验证**

Run:

```bash
TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build
grep -c "m3-card\|--primary:" dist/*.css
```

Expected: 构建成功；grep 输出 ≥ 2（tokens 与组件类已编译进产物）。

- [ ] **Step 4: 提交**

```bash
git add public/main.css tailwind.config.js
git commit -m "feat: 重建 CSS 管线，定义 Material 3 暖橙 tokens 与语义组件类

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 2: 暗色模式基础设施

**Files:**
- Modify: `Cargo.toml`（web-sys 增加 DomTokenList）
- Modify: `index.html`（防 FOUC 初始化脚本）
- Modify: `src/icons.rs`（增加 sun/moon 图标）
- Modify: `src/app.rs`（theme signal + 切换按钮）

- [ ] **Step 1: `Cargo.toml` 的 web-sys features 数组中追加 `"DomTokenList"`**

将

```toml
web-sys = { version = "0.3", features = [
  "Window", "Document", "Storage", "Blob", "BlobPropertyBag", "Url",
  "HtmlAnchorElement", "HtmlInputElement", "HtmlElement", "File", "FileList", "Element",
] }
```

改为

```toml
web-sys = { version = "0.3", features = [
  "Window", "Document", "Storage", "Blob", "BlobPropertyBag", "Url",
  "HtmlAnchorElement", "HtmlInputElement", "HtmlElement", "File", "FileList", "Element",
  "DomTokenList",
] }
```

- [ ] **Step 2: `index.html` 的 `<head>` 中（`<title>` 之后）插入防 FOUC 脚本**

```html
    <script>
      try {
        var t = localStorage.getItem('theme');
        if (t === 'dark' || (!t && matchMedia('(prefers-color-scheme: dark)').matches)) {
          document.documentElement.classList.add('dark');
        }
      } catch (e) {}
    </script>
```

- [ ] **Step 3: `src/icons.rs` 的 `paths()` match 中增加两个分支**（放在 `"scroll-text" =>` 分支之后、`_ => &[]` 之前）

```rust
        "sun" => &[
            "M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8z",
            "M12 2v2", "M12 20v2",
            "m4.93 4.93 1.41 1.41", "m17.66 17.66 1.41 1.41",
            "M2 12h2", "M20 12h2",
            "m6.34 17.66-1.41 1.41", "m19.07 4.93-1.41 1.41",
        ],
        "moon" => &["M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"],
```

- [ ] **Step 4: `src/app.rs` 增加 theme 状态**

在 `let file_ref = NodeRef::<leptos::html::Input>::new();` 之后插入：

```rust
    let dark_mode = RwSignal::new(
        document()
            .document_element()
            .map(|el| el.class_list().contains("dark"))
            .unwrap_or(false),
    );
    Effect::new(move |_| {
        let d = dark_mode.get();
        if let Some(el) = document().document_element() {
            let cl = el.class_list();
            if d {
                let _ = cl.add_1("dark");
            } else {
                let _ = cl.remove_1("dark");
            }
        }
        if let Some(storage) = window().local_storage().ok().flatten() {
            let _ = storage.set_item("theme", if d { "dark" } else { "light" });
        }
    });
```

- [ ] **Step 5: `src/app.rs` 顶栏加切换按钮**

将 `<div class="flex items-center gap-2">`（导出/导入按钮的容器）内、导出按钮之前插入：

```rust
                        <button
                            class="m3-icon-btn"
                            title="切换深色模式"
                            on:click=move |_| dark_mode.update(|v| *v = !*v)
                        >
                            {move || if dark_mode.get() {
                                view! { <Icon name="sun" class="h-5 w-5" /> }.into_any()
                            } else {
                                view! { <Icon name="moon" class="h-5 w-5" /> }.into_any()
                            }}
                        </button>
```

- [ ] **Step 6: 构建验证 + 提交**

Run: `TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build`
Expected: 构建成功（无 web-sys feature 缺失错误）。

```bash
git add Cargo.toml Cargo.lock index.html src/icons.rs src/app.rs
git commit -m "feat: 暗色模式基础设施（localStorage 持久化 + 防 FOUC + 切换按钮）

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 3: app.rs 视觉改造

**Files:**
- Modify: `src/app.rs`

- [ ] **Step 1: 头部常量与装饰元素**

替换 `HEADER_BTN` 常量：

```rust
const HEADER_BTN: &str = "m3-btn-outlined";
```

替换 `ALERT_CANCEL_CLASS` 常量：

```rust
const ALERT_CANCEL_CLASS: &str = "m3-btn-outlined";
```

两个导出/导入按钮的 `class=format!("{HEADER_BTN} hover:border-[#4c87bf] hover:text-[#4c87bf]")` 和 `class=format!("{HEADER_BTN} hover:border-[#1d5d3f] hover:text-[#1d5d3f]")` 均改为：

```rust
                            class=HEADER_BTN
```

两个装饰 blob：

- `bg-[#82eda6]/25` → `bg-earn-container/60`
- `bg-[#f6bbfd]/20` → `bg-primary-container/60`

日期行 `text-[#9274b1]` → `text-on-surface-variant`。

- [ ] **Step 2: 导入提示条**

import_error 提示条 class 改为：

```rust
                    <div class="mt-3 rounded-2xl bg-error-container px-4 py-2.5 font-display text-sm font-bold text-on-error-container">
```

import_success 提示条 class 改为：

```rust
                    <div class="mt-3 rounded-2xl bg-earn-container px-4 py-2.5 font-display text-sm font-bold text-on-earn-container">
```

- [ ] **Step 3: 孩子切换 chip**

选中/未选中分支（`format!` 中的条件字符串）改为：

```rust
                                            if credits.selected_kid_id.get() == Some(id) {
                                                "border-transparent bg-primary text-on-primary shadow-elevation-1"
                                            } else {
                                                "border-outline-variant bg-surface-container-lowest hover:bg-secondary-container"
                                            }
```

基础串中的 `border-2` 改为 `border`（即 `"flex items-center gap-2 rounded-full border py-1.5 pl-2 pr-4 font-display font-bold transition-all {}"`）。

头像圆圈条件字符串改为：

```rust
                                            if credits.selected_kid_id.get() == Some(id) { "bg-on-primary/15" } else { "bg-secondary-container" }
```

编辑按钮：`bg-white text-[#4c87bf] shadow hover:scale-110` → `bg-surface-container-lowest text-primary shadow-elevation-1 hover:scale-110`
删除按钮：`bg-white text-[#872020] shadow hover:scale-110` → `bg-surface-container-lowest text-error shadow-elevation-1 hover:scale-110`

「添加宝贝」按钮 class 改为：

```rust
                        class="flex items-center gap-1 rounded-full border border-dashed border-outline px-4 py-2 font-display font-bold text-primary transition-all hover:bg-secondary-container"
```

- [ ] **Step 4: 积分 hero 卡**

外层 `mt-6 overflow-hidden rounded-[2rem] bg-[#1d5d3f] p-6 text-white shadow-lg` → `mt-6 overflow-hidden rounded-[2rem] bg-primary p-6 text-on-primary shadow-elevation-2`。

内部颜色替换：

- 标签 `text-[#82eda6]` → `text-on-primary/80`
- 大数字 `text-[#ecc22e]` → `text-on-primary`
- 三处小标签 `text-white/70` → `text-on-primary/70`
- 累计赚得 `text-[#82eda6]` → `text-on-primary`
- 累计花掉 `text-[#f6bbfd]` → `text-on-primary`
- 今日打卡 `text-[#fdc068]` → `text-on-primary`

- [ ] **Step 5: 空状态与 Tabs**

空状态 div：`rounded-3xl border-2 border-dashed border-border bg-card/50 p-14 text-center` → `rounded-3xl border-2 border-dashed border-outline-variant bg-surface-container-low/50 p-14 text-center text-on-surface-variant`。

TabsList class：`grid h-auto w-full grid-cols-5 rounded-full bg-white p-1.5 shadow-sm` → `grid h-auto w-full grid-cols-5 rounded-full bg-surface-container-low p-1.5`。

5 个 TabsTrigger 的 `active_class` 统一改为 `"bg-primary text-on-primary shadow-elevation-1"`，`class` 统一改为 `"rounded-full py-2 font-display font-bold text-on-surface-variant"`。

删除确认对话框的「确定删除」按钮 class 改为 `"m3-btn-danger"`，`content_class="rounded-3xl"` 改为 `content_class=""`（删除该 prop 传参中的字符串，保留 prop 或改为空串均可，推荐直接删掉该 prop）。

- [ ] **Step 6: 构建验证 + 提交**

Run: `TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build && grep -rn "#1d5d3f\|#9274b1\|#ecc22e\|#f8622f\|#82eda6\|#f6bbfd\|#4c87bf\|#872020" src/app.rs`
Expected: 构建成功；grep 无输出。

```bash
git add src/app.rs
git commit -m "feat: app 顶栏/孩子切换/积分卡/Tabs 改造为 M3 风格

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 4: today_tasks.rs 改造

**Files:**
- Modify: `src/sections/today_tasks.rs`

- [ ] **Step 1: 标题区与「新任务」按钮**

`text-[#f8622f]`（今日赚到数字）→ `text-primary`。

「新任务」按钮 class 改为 `"m3-btn-filled"`。

- [ ] **Step 2: 任务卡片**

卡片 `format!` 的三个部分改为：

```rust
                                format!(
                                    "group relative flex items-center gap-3 p-4 transition-all duration-300 {} {}",
                                    if done {
                                        "rounded-2xl bg-earn text-on-earn shadow-elevation-1"
                                    } else {
                                        "m3-card hover:shadow-elevation-2"
                                    },
                                    if just_done.get() == Some(id) { "animate-pop-in" } else { "" }
                                )
```

图标方块条件：`"bg-white/15"` → `"bg-on-earn/15"`；`"bg-[#feffc9]"` → `"bg-secondary-container"`。

积分文字条件：完成态 `"text-[#ecc22e]"` → `"text-on-earn/80"`；未完成 `"text-[#f8622f]"` → `"text-primary"`。

打卡圆圈条件改为：

```rust
                                        if credits.is_task_done(id) {
                                            "border-transparent bg-on-earn text-earn"
                                        } else {
                                            "border-outline/50 text-transparent"
                                        }
```

编辑按钮：`bg-white text-[#4c87bf] shadow hover:scale-110` → `bg-surface-container-lowest text-primary shadow-elevation-1 hover:scale-110`
删除按钮：`bg-white text-[#872020] shadow hover:scale-110` → `bg-surface-container-lowest text-error shadow-elevation-1 hover:scale-110`

- [ ] **Step 3: 构建验证 + 提交**

Run: `TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build && grep -n "#" src/sections/today_tasks.rs`
Expected: 构建成功；grep 无输出。

```bash
git add src/sections/today_tasks.rs
git commit -m "feat: 今日打卡列表改造为 M3 卡片与打卡圆圈

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 5: wishes.rs 改造

**Files:**
- Modify: `src/sections/wishes.rs`

- [ ] **Step 1: 常量与「新心愿」按钮**

`ALERT_CANCEL_CLASS` 改为 `"m3-btn-outlined"`；`ALERT_ACTION_CLASS` 改为 `"m3-btn-filled"`。

「新心愿」按钮 class 改为 `"m3-btn-filled"`。

- [ ] **Step 2: 心愿卡片**

卡片 `format!` 改为：

```rust
                                format!(
                                    "group relative p-4 transition-all {} {}",
                                    if affordable { "m3-card ring-2 ring-primary" } else { "m3-card" },
                                    if celebrate.get() == Some(id) { "animate-wiggle" } else { "" }
                                )
```

图标方块 `bg-[#f3ecfa]` → `bg-secondary-container`。
价格文字 `text-[#9274b1]` → `text-on-surface-variant`。

兑换按钮 `format!` 改为：

```rust
                                            format!(
                                                "inline-flex h-9 items-center justify-center gap-1.5 whitespace-nowrap rounded-full px-4 font-display text-sm font-bold transition-all {}",
                                                if affordable {
                                                    "bg-primary text-on-primary shadow-elevation-1 hover:shadow-elevation-2"
                                                } else {
                                                    "bg-surface-container-highest text-on-surface-variant"
                                                }
                                            )
```

进度条轨道 `bg-muted` → `bg-surface-container-highest`；填充条件 `"bg-[#ecc22e]"` → `"bg-earn"`，`"bg-[#9274b1]/60"` → `"bg-primary/50"`。

编辑/删除小按钮：同 Task 4 的替换（`bg-white text-[#4c87bf] shadow hover:scale-110` → `bg-surface-container-lowest text-primary shadow-elevation-1 hover:scale-110`；`bg-white text-[#872020] shadow hover:scale-110` → `bg-surface-container-lowest text-error shadow-elevation-1 hover:scale-110`）。

确认对话框中消耗积分数 `text-[#f8622f]` → `text-primary`；`content_class="rounded-3xl"` 删除该 prop。

- [ ] **Step 3: 构建验证 + 提交**

Run: `TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build && grep -n "#" src/sections/wishes.rs`
Expected: 构建成功；grep 无输出。

```bash
git add src/sections/wishes.rs
git commit -m "feat: 心愿兑换改造为 M3 卡片、进度条与按钮

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 6: calendar_board.rs 改造

**Files:**
- Modify: `src/sections/calendar_board.rs`

- [ ] **Step 1: 卡片容器与月份导航**

两处 `rounded-3xl border-2 border-border bg-card p-5` → `m3-card p-5`。

上/下月按钮 class 均改为 `"m3-icon-btn h-9 w-9"`。

星期表头 `text-muted-foreground` → `text-on-surface-variant`；明细区空文案 `text-muted-foreground` → `text-on-surface-variant`；明细汇总 `text-muted-foreground` → `text-on-surface-variant`。

- [ ] **Step 2: 日期格子**

基础串改为：

```rust
                                    "relative flex min-h-[64px] flex-col items-center justify-start rounded-2xl px-1 py-1.5 transition-all sm:min-h-[76px] {}",
```

条件分支改为：

```rust
                                    if is_selected {
                                        "bg-primary text-on-primary shadow-elevation-1"
                                    } else if has_earned {
                                        ""
                                    } else {
                                        "bg-surface-container-low hover:bg-surface-container-high"
                                    }
```

热力 inline style 改为（用 CSS 变量表达透明度，暗色自动适配）：

```rust
                                let style = if !is_selected && has_earned {
                                    format!("background-color: rgb(var(--earn) / {:.2})", 0.10 + intensity * 0.25)
                                } else {
                                    String::new()
                                };
```

日期数字条件：`"text-white"` → `"text-on-primary"`；`"text-[#f8622f]"`（今天）→ `"text-primary"`；`"text-muted-foreground/50"` → `"text-on-surface-variant/50"`。

今天小圆点 `bg-[#f8622f]` → `bg-primary`。

赚得数字：选中 `"text-[#ecc22e]"` → `"text-on-primary"`；未选 `"text-[#1d5d3f]"` → `"text-earn"`。
花掉数字：选中 `"text-[#f6bbfd]"` → `"text-on-primary/80"`；未选 `"text-[#f8622f]"` → `"text-primary"`。

全勤/项数 chip 条件改为：

```rust
                                                        if s.checks >= task_count {
                                                            if is_selected { "bg-on-primary text-primary" } else { "bg-earn text-on-earn" }
                                                        } else if is_selected {
                                                            "bg-on-primary/20 text-on-primary"
                                                        } else {
                                                            "bg-surface-container-highest text-on-surface-variant"
                                                        }
```

- [ ] **Step 3: 图例与明细列表**

图例：色块 `bg-[#1d5d3f]/25` → `bg-earn/25`；`text-[#1d5d3f]` → `text-earn`；`text-[#f8622f]` → `text-primary`；全勤 chip `bg-[#ecc22e]/70 text-[#1d5d3f]` → `bg-earn text-on-earn`。

明细汇总：赚 `text-[#1d5d3f]` → `text-earn`；花 `text-[#f8622f]` → `text-primary`。

明细列表容器 `divide-y divide-border` → `divide-y divide-outline-variant`。

每行圆形图标：`bg-[#1d5d3f]/10 text-[#1d5d3f]` → `bg-earn-container text-on-earn-container`；`bg-[#f8622f]/10 text-[#f8622f]` → `bg-primary-container text-on-primary-container`。

每行金额：`text-[#1d5d3f]` → `text-earn`；`text-[#f8622f]` → `text-primary`。

- [ ] **Step 4: 构建验证 + 提交**

Run: `TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build && grep -n "#" src/sections/calendar_board.rs`
Expected: 构建成功；grep 无输出。

```bash
git add src/sections/calendar_board.rs
git commit -m "feat: 日历看板改造为 M3 卡片与热力色阶

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 7: history.rs + charts.rs + charts_svg.rs 改造

**Files:**
- Modify: `src/sections/history.rs`
- Modify: `src/sections/charts.rs`
- Modify: `src/charts_svg.rs`（含测试更新）

- [ ] **Step 1: history.rs**

空状态 section class 改为 `"rounded-3xl border-2 border-dashed border-outline-variant bg-surface-container-low/50 p-10 text-center text-on-surface-variant"`。

列表容器 `divide-y divide-border overflow-hidden rounded-3xl border-2 border-border bg-card` → `m3-card divide-y divide-outline-variant overflow-hidden`。

每行圆形图标：`bg-[#1d5d3f]/10 text-[#1d5d3f]` → `bg-earn-container text-on-earn-container`；`bg-[#f8622f]/10 text-[#f8622f]` → `bg-primary-container text-on-primary-container`。

日期 `text-muted-foreground` → `text-on-surface-variant`。

金额：`text-[#1d5d3f]` → `text-earn`；`text-[#f8622f]` → `text-primary`。

- [ ] **Step 2: charts.rs**

两处卡片 `rounded-3xl border-2 border-border bg-card p-5` → `m3-card p-5`。
两处说明文字 `text-muted-foreground` → `text-on-surface-variant`。

第一张图的说明文案与新配色同步（余额条已改主题橙）：`"金色是当前攒下的积分，彩色是每个心愿需要的积分，一眼看出还差多少"` → `"橙色是当前攒下的积分，彩色是每个心愿需要的积分，一眼看出还差多少"`。

- [ ] **Step 3: charts_svg.rs 颜色改 CSS 变量**

常量替换：

```rust
pub const COMPARE_COLORS: [&str; 6] =
    ["#984800", "#2f6b3c", "#0b57d0", "#a63c66", "#6d5a00", "#4c4aa8"];
```

`compare_chart_svg` 中：

- 余额行 `color: "#ecc22e"` → `color: "var(--primary)"`
- 网格线 `stroke="#e8e0cd"` → `stroke="var(--outline-variant)"`
- X 刻度与行标签 `fill="#1d5d3f"` → `fill="var(--on-surface-variant)"`（共 2 处：刻度 text、label text）
- 行数值 `fill="#1d5d3f"`（font-weight="800" 那处）→ `fill="var(--on-surface)"`

`week_chart_svg` 中：

- 网格线 `stroke="#e8e0cd"` → `stroke="var(--outline-variant)"`
- Y 刻度与日期标签 `fill="#1d5d3f"` → `fill="var(--on-surface-variant)"`（2 处）
- 赚得柱 `"#1d5d3f"` → `"var(--earn)"`；花掉柱 `"#f8622f"` → `"var(--primary)"`

- [ ] **Step 4: 更新 charts_svg 测试断言**

`compare_chart_has_one_row_per_wish_plus_balance` 中：

```rust
        // 余额条用主题色
        assert!(svg.contains("fill=\"var(--primary)\""));
```

（替换原 `assert!(svg.contains("fill=\"#ecc22e\""));` 及其注释）

`week_chart_renders_7_days_and_bars` 中：

```rust
        assert!(svg.contains("fill=\"var(--earn)\""));
        assert!(svg.contains("fill=\"var(--primary)\""));
```

（替换原 `#1d5d3f` / `#f8622f` 两条断言）

- [ ] **Step 5: 测试 + 构建 + 提交**

Run: `cargo test && TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build`
Expected: 测试全绿；构建成功。

```bash
git add src/sections/history.rs src/sections/charts.rs src/charts_svg.rs
git commit -m "feat: 记录/图表改造为 M3 配色，SVG 图表颜色接入主题变量

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 8: 弹窗与通用组件改造

**Files:**
- Modify: `src/components/dialog.rs`
- Modify: `src/components/kid_dialog.rs`
- Modify: `src/components/item_dialog.rs`
- Modify: `src/components/tabs.rs`

- [ ] **Step 1: dialog.rs**

`CONTENT_BASE` 改为：

```rust
const CONTENT_BASE: &str = "bg-surface-container-high text-on-surface fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-[28px] p-6 shadow-elevation-3 duration-200 outline-none sm:max-w-lg";
```

遮罩 `bg-black/50` → `bg-black/40`。

关闭按钮 class 改为 `"m3-icon-btn absolute top-3 right-3 h-8 w-8"`。

- [ ] **Step 2: kid_dialog.rs**

三个常量改为：

```rust
pub(crate) const INPUT_CLASS: &str = "m3-input";
pub(crate) const LABEL_CLASS: &str = "font-display text-sm font-bold text-on-surface-variant";
pub(crate) const SAVE_BTN_CLASS: &str = "m3-btn-filled w-full";
```

头像选择按钮条件：`"bg-[#ecc22e] scale-110 shadow-sm"` → `"bg-primary-container ring-2 ring-primary scale-110"`；`"bg-muted hover:bg-[#feffc9]"` → `"bg-surface-container-low hover:bg-secondary-container"`。

`content_class="rounded-3xl sm:max-w-md"` → `content_class="sm:max-w-md"`。

- [ ] **Step 3: item_dialog.rs**

图标选择按钮条件：`"bg-[#ecc22e] scale-110 shadow-sm"` → `"bg-primary-container ring-2 ring-primary scale-110"`；`"bg-muted hover:bg-[#feffc9]"` → `"bg-surface-container-low hover:bg-secondary-container"`。

`content_class="rounded-3xl sm:max-w-md"` → `content_class="sm:max-w-md"`。

- [ ] **Step 4: tabs.rs**

TabsList 基础 class 中的 `"bg-muted text-muted-foreground inline-flex h-9 w-fit items-center justify-center rounded-lg p-[3px] {class}"` 改为：

```rust
            "text-on-surface-variant inline-flex h-9 w-fit items-center justify-center rounded-full p-[3px] {class}"
```

`TRIGGER_BASE` 改为（去掉 rounded-md，由调用方传 rounded-full）：

```rust
const TRIGGER_BASE: &str = "inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 border border-transparent px-2 py-1 text-sm font-medium whitespace-nowrap transition-[color,box-shadow] disabled:pointer-events-none disabled:opacity-50";
```

- [ ] **Step 5: 构建验证 + 提交**

Run: `TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build && grep -rn "text-muted-foreground\|bg-muted\|bg-card\|border-border\|bg-background\|bg-accent\|shadow-xs\|#ecc22e\|#feffc9" src/`
Expected: 构建成功；grep 无输出（全仓库旧 token 清零）。

```bash
git add src/components/
git commit -m "feat: 弹窗与表单控件改造为 M3 对话框/输入框样式

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 9: 端到端验证与收尾

**Files:** 无新改动（如发现问题则修复）

- [ ] **Step 1: 全量测试与 release 构建**

Run: `cargo test && TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build --release`
Expected: 测试全绿；release 构建成功（与 CI 管线一致）。

- [ ] **Step 2: 浏览器实测**

Run（后台）: `TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk serve`

用 Playwright 打开 `http://localhost:8080`，逐个检查：

1. 打卡 / 心愿 / 日历 / 图表 / 记录 5 个 tab 亮色截图
2. 点击顶栏月亮按钮切暗色，再截图 5 个 tab
3. 打开「添加宝贝」「新任务」弹窗截图
4. 刷新页面确认暗色偏好持久化（localStorage `theme`）

检查要点：文字对比度可读、无白色块状残留（旧 bg-white）、打卡/兑换交互正常。

- [ ] **Step 3: 修复实测发现的问题（如有），然后推送**

```bash
git push
```

Expected: CI（deploy.yml）自动构建并发布到 GitHub Pages，`https://zpvan.github.io/deb-credit/` 几分钟后更新为新风格。
