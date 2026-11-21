//! `LogViewer` infrastructure layer.
//!
//! Platform-neutral building blocks (line index, parsers, format detection)
//! plus the platform readers implementing
//! [`LogSource`](logviewer_application::ports::LogSource), each behind a Cargo
//! feature:
//!
//! - `native`: desktop reader (memory-mapped file), used by `logviewer-server`;
//! - `browser`: web reader (`Blob.slice`), used by `logviewer-wasm`.

pub mod detection;
pub mod index;
pub mod parsers;
pub mod reader;

// Layers wired in advance; the imports go once real code uses them.
use logviewer_application as _;
use logviewer_domain as _;
