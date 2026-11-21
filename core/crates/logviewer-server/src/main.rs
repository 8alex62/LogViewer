//! `LogViewer` desktop sidecar.
//!
//! Spawned by the desktop UI as a child process, bound to `127.0.0.1` with a
//! shared token; exposes the Core over gRPC. This is the composition root:
//! it picks the `native` infrastructure and hands it to the use cases.
//!
//! The gRPC transport is not wired yet (walking skeleton).

mod config;
mod grpc;

// Composition root: every layer is linked; the imports go once real code uses them.
use logviewer_application as _;
use logviewer_domain as _;
use logviewer_infrastructure as _;

fn main() {
    println!("logviewer-server {}", env!("CARGO_PKG_VERSION"));
}
