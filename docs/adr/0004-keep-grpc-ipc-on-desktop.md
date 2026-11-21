# 0004. Keep gRPC IPC on desktop rather than direct calls (FFI)

- Status: accepted
- Date: 2026-11-21 (decision taken on 2026-11-14)

## Context

On desktop the UI spawns the Core as a child process (sidecar), bound to `127.0.0.1` on a random port with a shared token, and talks to it over gRPC (ADR 0002). An alternative is to load the Core as a native library and call it directly through FFI (`csbindgen` or `uniffi`), removing the process boundary.

## Decision

**Keep gRPC over loopback on desktop.**

Reasons:

1. **One contract**: desktop and web use the same protobuf messages; gRPC is their natural transport on desktop. FFI would add native libraries to ship per OS and architecture, and an async boundary to manage by hand.
2. **Isolation**: if the Core crashes or misbehaves on untrusted content, the UI stays alive and can restart the sidecar.
3. **Low cost**: a loopback call is sub-millisecond, and `GetLines` returns pages of lines, so round trips are few.

## Consequences

- The UI manages the Core process: start, token, port discovery, restart, and stop on exit.
- The Core listens on loopback only and rejects requests without the shared token.
- **Door left open**: thanks to the Clean Architecture (ADR 0006) the application layer does not depend on the transport. Revisit only on measured evidence: `GetLines` p99 above 16 ms (one frame) while scrolling, measured with `criterion` benchmarks plus an end-to-end test.
