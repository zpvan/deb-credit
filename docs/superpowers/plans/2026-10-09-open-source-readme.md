# 开源 README 与 GitHub About 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 deb-credit 作为独立开源项目对外展示：英文 README、MIT LICENSE、GitHub About。

**Architecture:** 纯文档任务。重写 `README.md`（英文为主，不提"Rust 版/重写"），新增标准 MIT `LICENSE`，用 `gh repo edit` 设置仓库 description 与 topics，提交并推送到 main。

**Tech Stack:** Markdown、gh CLI

**Spec:** `docs/superpowers/specs/2026-10-09-open-source-readme-design.md`

---

### Task 1: 重写 README.md 并新增 LICENSE

**Files:**
- Modify: `README.md`（整体重写）
- Create: `LICENSE`

- [ ] **Step 1: 重写 `README.md`**

用以下内容**整体替换**现有 `README.md`：

````markdown
# Kids Credit Station · 宝贝积分站

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A kid-friendly credit & reward tracker: children check in daily tasks to earn
credits, then redeem them for wishes. A single-page app that runs entirely in
the browser — no account, no server, all data stays on the device.

## Features

- **Multiple kids** — manage a separate credit account for each child, with name and emoji avatar
- **Daily check-in** — define earn tasks (e.g. "Brush teeth", "Finish homework") with custom credit values, and check them off each day
- **Calendar board** — review check-ins at a glance on a monthly calendar
- **Statistics** — charts of credits earned over time
- **Wish shop** — create wish items with credit costs; kids redeem a wish once they've saved enough
- **History** — a full transaction log of every credit earned and spent
- **Backup & restore** — export and import all data as JSON
- **Offline & private** — everything is stored in the browser's localStorage; nothing ever leaves the device

## Tech Stack

- [Leptos](https://leptos.dev/) 0.8 (client-side rendering) compiled to WebAssembly
- [Tailwind CSS](https://tailwindcss.com/) v3
- No backend — the browser is the whole runtime

## Getting Started

### Prerequisites

- Rust with the `wasm32-unknown-unknown` target
- [trunk](https://trunkrs.dev/)
- Tailwind CSS v3 CLI

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
npm install --no-save tailwindcss@3.4.19   # or use the standalone CLI v3.4.x
```

### Develop

```bash
TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk serve
```

### Test

```bash
cargo test   # data-layer unit tests
```

### Build

```bash
TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build --release
```

The static site is emitted to `dist/` and can be hosted anywhere.

## Data & Privacy

All data — kids, tasks, wishes, and transactions — lives in the browser's
localStorage. Use the in-app export to download a JSON backup, and import it
on another device or browser to migrate.

## License

[MIT](LICENSE)
````

注意：全文不得出现 "Rust 版"、"重写"、"rewrite"、"React" 字样。

- [ ] **Step 2: 创建 `LICENSE`**

写入标准 MIT 许可证全文：

```text
MIT License

Copyright (c) 2026 zpvan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

- [ ] **Step 3: 校验文案**

Run:

```bash
grep -inE "rust 版|重写|rewrite|react" README.md
```

Expected: 无输出（exit code 1，未匹配到任何行）。

```bash
head -1 README.md && head -3 LICENSE
```

Expected: 第一行是 `# Kids Credit Station · 宝贝积分站`，LICENSE 前三行为 `MIT License`、空行、`Copyright (c) 2026 zpvan`。

- [ ] **Step 4: 提交**

```bash
git add README.md LICENSE
git commit -m "docs: 重写英文 README，新增 MIT LICENSE

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

### Task 2: 设置 GitHub About 并推送

**Files:** 无文件改动，仅 gh CLI 操作与 git push。

- [ ] **Step 1: 确认 gh 已登录**

Run:

```bash
gh auth status
```

Expected: 显示已登录 github.com。若未登录，请用户在终端执行 `! gh auth login` 后重试。

- [ ] **Step 2: 设置 description 与 topics**

Run:

```bash
gh repo edit --description "A kid-friendly credit & reward tracker — check in daily tasks, earn credits, and redeem wishes. Runs entirely in the browser." \
  --add-topic kids --add-topic reward-system --add-topic check-in \
  --add-topic webassembly --add-topic leptos --add-topic tailwindcss \
  --add-topic single-page-app
```

Expected: 无报错输出。

- [ ] **Step 3: 推送并验证**

Run:

```bash
git push
gh repo view --json description,repositoryTopics -q '{description: .description, topics: [.repositoryTopics[].name]}'
```

Expected: push 成功；输出中 description 与 Step 2 的文案一致，topics 包含全部 7 个标签。
