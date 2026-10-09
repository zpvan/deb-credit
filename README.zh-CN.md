# Kids Credit Station

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

[English](README.md) | **简体中文**

一个给孩子用的积分奖励打卡应用：孩子每天完成任务打卡攒积分，攒够了就兑换心愿。单页应用，完全运行在浏览器里——无需注册、无需服务器，所有数据都保存在本机。

## 功能特性

- **多孩子管理** — 为每个孩子建立独立积分账户，自定义名字和 emoji 头像
- **每日打卡** — 自定义赚积分任务（如"刷牙"、"完成作业"）及对应分值，每天打卡
- **日历视图** — 在月度日历上一目了然地回顾打卡情况
- **积分统计** — 图表展示积分随时间的变化
- **心愿商店** — 创建心愿物品并设置积分价格，攒够积分即可兑换
- **历史记录** — 每一笔积分收支的完整流水
- **备份与恢复** — 全部数据可导出为 JSON，也可导入恢复
- **离线且私密** — 数据全部存于浏览器 localStorage，不会离开本机

## 技术栈

- [Leptos](https://leptos.dev/) 0.8（客户端渲染）编译为 WebAssembly
- [Tailwind CSS](https://tailwindcss.com/) v3
- 无后端——浏览器即全部运行时

## 快速开始

### 环境要求

- Rust（含 `wasm32-unknown-unknown` target）
- [trunk](https://trunkrs.dev/)
- Tailwind CSS v3 CLI

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
npm install --no-save tailwindcss@3.4.19   # 或使用 standalone CLI v3.4.x
```

### 开发

```bash
TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk serve
```

### 测试

```bash
cargo test   # 数据层单元测试
```

### 构建

```bash
TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build --release
```

静态站点输出到 `dist/`，可托管到任意静态服务器。

## 数据与隐私

所有数据——孩子、任务、心愿、流水——都保存在浏览器的 localStorage 中。可在应用内导出 JSON 备份，并在其他设备或浏览器上导入迁移。

## 许可证

[MIT](LICENSE)
