# Architecture Decision Records

One file per decision, numbered, never rewritten once accepted: a new ADR supersedes an old one.

Write one when a decision is structuring and hard to undo (technology, architecture, product principle), not for every detail. Add it to the index below and, if it changes the specs, update them in Fibery too.

## Index

| ADR | Decision | Status |
|---|---|---|
| [0001](0001-rust-core-csharp-ui.md) | Rust Core and C# / Avalonia UI | accepted |
| [0002](0002-grpc-over-jsonrpc.md) | gRPC and Protocol Buffers rather than JSON-RPC | accepted |
| [0003](0003-web-core-in-wasm-no-upload.md) | Web: Core compiled to WASM in a Web Worker, no upload | accepted |
| [0004](0004-keep-grpc-ipc-on-desktop.md) | Keep gRPC IPC on desktop rather than FFI | accepted |
| [0005](0005-privacy-logs-never-leave-the-machine.md) | Privacy: log files never leave the user's machine | accepted |
| [0006](0006-clean-architecture-crates.md) | Core: Clean Architecture crates, workspace in `core/` | accepted |
| [0007](0007-no-macos-desktop-build.md) | No macOS desktop build; macOS users use the web app | accepted |

## Template

```markdown
# NNNN. Title

- Status: proposed | accepted | superseded by NNNN
- Date: YYYY-MM-DD

## Context
What forces the decision.

## Decision
What we do.

## Consequences
What becomes easier or harder.
```
