# 0001. Rust Core and C# / Avalonia UI

- Status: accepted
- Date: 2026-11-21

## Context

LogViewer must open multi-GB log files, show the first lines of a 1 GB file in under a second, and stay responsive while it indexes, filters and searches. Log content is untrusted input. The application ships on desktop (Windows, Linux) and as a static web app, and both must behave the same.

Two kinds of work have very different needs:

- **Log processing** (reading by ranges, line index, parsing, filtering, search) is CPU- and memory-bound and must never load a whole file in memory.
- **The UI** (tabs, virtualized list, filters, themes, accessibility, localization) is rich and benefits from a mature MVVM framework.

## Decision

Split the application in two components that talk through a Protocol Buffers contract (see ADR 0002):

- **Core in Rust**: reads, indexes, parses, filters and searches logs.
- **Client in C# with Avalonia** (MVVM, ReactiveUI): the UI, a client of the Core.

The same Core runs as a desktop sidecar process and, compiled to WebAssembly, inside a Web Worker in the browser (ADR 0003).

Reasons:

1. **Performance and memory control (Rust)**: no garbage collector pauses on huge files, memory-mapped reads, explicit control of allocations, and the `regex` crate runs in linear time, so a hostile pattern or line cannot trigger catastrophic backtracking. Memory safety without `unsafe` matters on untrusted content.
2. **One Core for desktop and web**: Rust compiles to `wasm32-unknown-unknown`, so the domain, use cases, index and parsers are written once and run in both modes.
3. **Existing C# / Avalonia skills**: the UI is built with a stack we already master, and Avalonia targets desktop and the browser (WASM) with a single shared UI.
4. **Learning goal**: the project is also a way to learn Rust and to practise a clean polyglot architecture.

## Alternatives considered

- **All Rust** (Tauri, Slint, egui): one language, but a less mature MVVM ecosystem for a rich desktop + web UI, and no existing expertise.
- **All C#**: one language and great tooling, but less control over memory on multi-GB files, and running heavy processing in .NET WASM in the browser is heavier than a dedicated Rust WASM module.

## Consequences

- Two toolchains (Cargo, .NET), two CI workflows (`core.yml`, `client.yml`) and two sets of conventions to maintain.
- The contract in `/proto` becomes the single source of truth between the two sides; changes to it must stay backward compatible (`buf` breaking-change checks).
- On desktop the Core runs in its own process: a crash on bad content does not take the UI down (see ADR 0004).
- Contributors need some knowledge of both languages, mitigated by the strict layer boundaries (ADR 0006).
