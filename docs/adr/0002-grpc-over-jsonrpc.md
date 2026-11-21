# 0002. gRPC and Protocol Buffers rather than JSON-RPC

- Status: accepted
- Date: 2026-11-21

## Context

The UI (C#) and the Core (Rust) are separate components written in different languages (ADR 0001). They need a contract that:

- stays in sync between the two languages as the API grows;
- supports server streams: indexing progress, search matches and live tailing push many messages for one request;
- returns pages of log lines efficiently while the user scrolls;
- also works in the browser, where the Core runs in a Web Worker and there is no network transport (ADR 0003).

## Decision

Define the API once in Protocol Buffers v3 under `/proto/logviewer/v1/` and serve it over **gRPC** on desktop (`tonic` in Rust, `Grpc.Net.Client` in C#). In the browser, the same protobuf messages are wrapped in an `Envelope` and travel through `postMessage` (`worker.proto`).

Reasons:

1. **Typed contract shared by both sides**: code is generated from the same `.proto` for Rust (`prost` / `tonic`) and C# (`Grpc.Tools`, `Google.Protobuf`), so the two sides cannot silently drift.
2. **Native streaming**: server streaming covers `GetIndexProgress`, `Search` and `Tail` without inventing a protocol.
3. **Performance**: protobuf is a compact binary format, cheaper to encode and decode than JSON for pages of lines.
4. **Same messages on the web**: the worker transport reuses the protobuf messages, so only the transport differs between desktop and web.

## Alternatives considered

- **JSON-RPC** (over stdio or HTTP): simple and easy to debug, but no schema or code generation (types written twice by hand), streaming must be designed ad hoc, and JSON is heavier for large pages.
- **REST / HTTP + JSON**: same drawbacks, plus a resource model that fits RPC-style calls poorly.

## Consequences

- `buf` lints the contract and, once the first version is on `develop`, blocks breaking changes in CI.
- Code generation needs `protoc` (or an equivalent) at build time, in `build.rs` of the presentation crates and in the C# build.
- Names follow the `buf` STANDARD rules (`LogViewerService`, `*Request` / `*Response`).
- Debugging the wire is less direct than JSON; tools such as `grpcurl` help.
