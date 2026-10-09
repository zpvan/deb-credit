# 开源项目 README 与 GitHub About 设计

日期：2026-10-09

## 目标

把 deb-credit 作为一个独立开源项目对外展示：重写 README、补充 LICENSE、设置 GitHub About（description + topics）。**不强调"Rust 版/重写"**，就是一个普通的开源项目。

## 决策记录

| 决策点 | 结论 |
|---|---|
| README 语言 | 英文为主（应用界面为中文，正文可提及中文名） |
| 开源许可证 | MIT，新增 `LICENSE` 文件，Copyright (c) 2026 zpvan |
| 应用截图 | 暂不放，用文字描述功能 |
| 在线演示 | 暂不部署 GitHub Pages，不放演示链接 |
| 英文项目名 | **Kids Credit Station**（"宝贝积分站"直译），标题格式 `# Kids Credit Station · 宝贝积分站` |

## 交付物

### 1. `README.md`（重写，英文）

结构：

1. **标题 + 徽章**：MIT License 徽章（shields.io）
2. **简介**：一段话——给孩子打卡攒积分、兑换心愿的单页应用
3. **Features**：多孩子管理 / 今日任务打卡 / 日历打卡视图 / 积分统计图表 / 心愿商店兑换 / 历史记录 / JSON 备份与恢复 / 纯本地存储（离线可用、隐私安全）
4. **Tech Stack**：Leptos 0.8 (CSR) + WebAssembly + Tailwind CSS v3（平实陈述技术栈，不提"重写自 React 版"）
5. **Getting Started**：环境要求（Rust + wasm32 target、trunk、tailwindcss v3）、开发、测试、构建命令（沿用现有 README 的命令）
6. **Data & Privacy**：数据存浏览器 localStorage；JSON 导出/导入备份
7. **License**：MIT

### 2. `LICENSE`（新增）

标准 MIT 许可证文本，Copyright (c) 2026 zpvan。

### 3. GitHub About（通过 `gh repo edit` 设置）

- **Description**：*"A kid-friendly credit & reward tracker — check in daily tasks, earn credits, and redeem wishes. Runs entirely in the browser."*
- **Topics**：`kids` `reward-system` `check-in` `webassembly` `leptos` `tailwindcss` `single-page-app`

## 约束

- README 与 About 文案中不出现 "Rust 版"、"重写"、"rewrite" 等表述；技术栈仅作事实陈述。
- 备份兼容说明：现有 README 提到"由 React 版 deb-credit 重写而来，备份 JSON 互相兼容"——删去重写历史，仅保留 JSON 备份/导入功能说明。
- 改动提交到 main 分支并推送到 GitHub。

## 验证

- `gh repo view` 确认 description 与 topics 生效。
- README 在 GitHub 上渲染正常（标题、徽章、列表）。
