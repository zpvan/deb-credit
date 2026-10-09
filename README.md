# Kids Credit Station

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**English** | [简体中文](README.zh-CN.md)

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
