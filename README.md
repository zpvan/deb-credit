# 宝贝积分站（Rust 版）

给孩子打卡攒积分、兑换心愿的单页应用。Leptos 0.8 (CSR) + WASM + Tailwind CSS v3，
数据存于浏览器 localStorage。由 React 版 deb-credit 重写而来，备份 JSON 互相兼容。

## 开发

需要：Rust（含 wasm32-unknown-unknown target）、trunk、tailwindcss v3。

    rustup target add wasm32-unknown-unknown
    cargo install trunk --locked
    npm install --no-save tailwindcss@3.4.19   # 或使用 standalone CLI v3.4.x

    TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk serve

## 测试与构建

    cargo test                                   # 数据层单元测试
    TRUNK_TOOLS_TAILWINDCSS=$PWD/node_modules/.bin/tailwindcss trunk build --release
