//! `LogViewer` core server.
//!
//! Spawned by the UI (desktop) as a child process; exposes `logviewer-core`
//! over RPC. The transport is not wired yet: this is the walking skeleton.

fn main() {
    println!("logviewer-server {}", logviewer_core::VERSION);
}
