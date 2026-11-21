# LogViewer

[![Core CI](https://github.com/8alex62/LogViewer/actions/workflows/core.yml/badge.svg?branch=develop)](https://github.com/8alex62/LogViewer/actions/workflows/core.yml?query=branch%3Adevelop)

A simple tool to look at log files in a fancy way

## Repository layout

| Path | Content |
|---|---|
| `core/` | Rust Core (Cargo workspace, Clean Architecture crates) |
| `proto/` | Protocol Buffers contract shared by the Core and the client |
| `docs/` | Architecture summary and ADRs |
| `samples/` | Synthetic log files for tests and demos |

## Core: build and test

```bash
cd core
cargo build
cargo test --workspace
```

## AI agent tooling

Optional: install [Repowise](https://docs.repowise.dev) (`uv tool install repowise`, then `repowise init` at the repo root) to give AI coding agents a local index of the codebase; `.claude/CLAUDE.md` and `.github/copilot-instructions.md` expect it.
