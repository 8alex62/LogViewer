# 0006. Core: Clean Architecture split into Cargo crates, workspace in `core/`

- Status: accepted
- Date: 2026-11-21 (decision taken on 2026-11-15, US-64)
- Supersedes: the initial workspace at the repository root with a `core` library and a `server` binary (US-58)

## Context

The first Cargo workspace lived at the repository root with two crates, `core` (library) and `server` (binary). The repository is polyglot (Rust Core, C# client, shared `/proto`), and the Core must run in two hosts (desktop sidecar, browser worker) with platform-specific readers and transports. Without enforced boundaries, platform and transport code tends to leak into the business logic, and the WASM build breaks.

## Decision

Move the Cargo workspace to `core/` (with its own `rust-toolchain.toml`) and split the Core into one crate per Clean Architecture layer, under `core/crates/`:

| Crate | Layer | May depend on |
|---|---|---|
| `logviewer-domain` | Entities (`LogEntry`, `LogLevel`, `LogFormat`, `Filter`) | nothing |
| `logviewer-application` | Use cases and ports (`LogSource`, `LogParser`) | domain |
| `logviewer-infrastructure` | Line index, parsers, detection; readers behind features `native` / `browser` | application, domain |
| `logviewer-server` (bin) | Desktop presentation: gRPC, composition root | all, `native` |
| `logviewer-wasm` (cdylib) | Web presentation: worker entry point, composition root | all, `browser` |

Rules:

1. Dependencies only point inward; the domain has no dependency at all.
2. Use cases only see files through the `LogSource` port, so they run unchanged on desktop and in WASM.
3. Platform crates (`memmap2`, `notify`, `web-sys`) only appear in `logviewer-infrastructure`, behind a feature.
4. Transport crates (`tonic`, `prost`, `wasm-bindgen`) only appear in the presentation crates.
5. Libraries return typed errors (`thiserror`); only binaries may use `anyhow`.

## Consequences

- Layer boundaries are enforced by Cargo: a crate cannot use another one it does not declare, and the dependency lists are easy to review.
- The repository root stays free for `/proto`, `client/`, `docs/` and `samples/`.
- More boilerplate (five manifests) and commands run from `core/` (CI `working-directory`, rust-analyzer `linkedProjects`).
- CI builds the WASM crate on every change, so platform code leaking into a shared crate is caught early.
